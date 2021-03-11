use crate::source_code::SourceSpan;
use crate::token::*;

#[derive(Debug, PartialEq)]
pub enum LexLuthorError {
  UnexpectedCharacter {
    source_span: SourceSpan,
    message: String,
  },
  InvalidIdentifier {
    source_span: SourceSpan,
    message: String,
  },
}

#[derive(Debug)]
pub struct LexLuthor {
  source_code: String,
  line: usize,
  column: usize,
  position: usize,
  character: char,
}

impl LexLuthor {
  pub fn new(source_code: String) -> LexLuthor {
    let mut lex_luthor = LexLuthor {
      source_code,
      line: 1,
      column: 0,
      position: 0,
      character: '\0',
    };

    lex_luthor.read_character();

    lex_luthor
  }

  fn current_source_span(&self) -> SourceSpan {
    SourceSpan {
      line: self.line,
      column: self.column,
    }
  }

  fn has_characters_to_lex(&self) -> bool {
    self.position <= self.source_code.len()
  }

  fn peek(&self) -> Option<char> {
    self.source_code.chars().nth(self.position)
  }

  fn next_character_is(&self, expected_character: char) -> bool {
    match self.peek() {
      None => false,
      Some(character) => character == expected_character,
    }
  }

  fn read_character(&mut self) {
    match self.source_code.chars().nth(self.position) {
      None => self.character = '\0',
      Some(character) => {
        self.character = character;

        self.column += 1;

        if self.character == '\n' {
          self.line += 1;
          self.column = 0;
        }
      }
    }

    self.position += 1;
  }

  fn read_identifier_or_keyword(&mut self) -> Result<String, LexLuthorError> {
    let start = self.position - 1;

    while self.character.is_digit(10) || self.character.is_alphabetic() || self.character == '_' {
      self.read_character();
    }

    let identifier_or_keyword: String = self
      .source_code
      .chars()
      .skip(start)
      .take(self.position - start)
      .collect();

    if identifier_or_keyword.len() == 1 {
      return Ok(identifier_or_keyword);
    }

    for (index, character) in identifier_or_keyword.chars().enumerate() {
      if (character.is_digit(10) || character == '_')
        && !matches!(identifier_or_keyword.chars().nth(index + 1), Some(character) if character.is_alphabetic())
      {
        return Err(LexLuthorError::InvalidIdentifier {
          message: format!(
            "{} is not a valid identifier, {} must be followed by a letter",
            identifier_or_keyword, character
          ),
          source_span: self.current_source_span(),
        });
      }
    }

    Ok(identifier_or_keyword)
  }

  fn skip_whitespace(&mut self) {
    while self.character.is_ascii_whitespace() {
      self.read_character();
    }
  }

  fn next_token(&mut self) -> Result<Token, LexLuthorError> {
    self.skip_whitespace();

    let token = match self.character {
      '{' => Token::LeftBrace(self.current_source_span()),
      '}' => Token::RightBrace(self.current_source_span()),
      '[' => Token::LeftBracket(self.current_source_span()),
      ']' => Token::RightBracket(self.current_source_span()),
      ',' => Token::Comma(self.current_source_span()),
      '+' => Token::Plus(self.current_source_span()),
      '-' => Token::Minus(self.current_source_span()),
      '/' => Token::Slash(self.current_source_span()),
      '*' => {
        if self.next_character_is('*') {
          self.read_character();
          Token::StarStar(self.current_source_span())
        } else {
          Token::Star(self.current_source_span())
        }
      }
      '%' => {
        if self.next_character_is('%') {
          self.read_character();
          Token::PercentPercent(self.current_source_span())
        } else {
          Token::Percent(self.current_source_span())
        }
      }
      '=' => Token::Equal(self.current_source_span()),
      '!' if self.next_character_is('=') => Token::NotEqual(self.current_source_span()),
      '<' => {
        if self.next_character_is('=') {
          self.read_character();
          Token::LessThanOrEqual(self.current_source_span())
        } else {
          Token::LessThan(self.current_source_span())
        }
      }
      '>' => {
        if self.next_character_is('=') {
          self.read_character();
          Token::GreaterThanOrEqual(self.current_source_span())
        } else {
          Token::GreaterThan(self.current_source_span())
        }
      }
      '&' => Token::Ampersand(self.current_source_span()),
      '|' => Token::Pipe(self.current_source_span()),
      '!' => Token::Bang(self.current_source_span()),
      '(' => Token::LeftParen(self.current_source_span()),
      ')' => Token::RightParen(self.current_source_span()),
      character if character.is_alphabetic() || character == '_' => {
        let identifier_or_keyword = self.read_identifier_or_keyword()?;
        token_from_identifier_or_keyword(identifier_or_keyword, self.current_source_span())
      }
      character => {
        self.read_character();
        return Err(LexLuthorError::UnexpectedCharacter {
          source_span: self.current_source_span(),
          message: format!("unexpected character {}", character),
        });
      }
    };

    self.read_character();

    Ok(token)
  }

  pub fn lex(&mut self) -> Result<Vec<Token>, Vec<LexLuthorError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    while self.has_characters_to_lex() {
      match self.next_token() {
        Ok(token) => tokens.push(token),
        Err(error) => errors.push(error),
      }
    }

    if !errors.is_empty() {
      Err(errors)
    } else {
      tokens.push(Token::Eof);
      Ok(tokens)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn recognizes_tokens() {
    let test_cases = vec![
      (
        "{",
        vec![
          Token::LeftBrace(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "}",
        vec![
          Token::RightBrace(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "[",
        vec![
          Token::LeftBracket(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "]",
        vec![
          Token::RightBracket(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        ",",
        vec![Token::Comma(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "+",
        vec![Token::Plus(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "-",
        vec![Token::Minus(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "/",
        vec![Token::Slash(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "*",
        vec![Token::Star(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "**",
        vec![
          Token::StarStar(SourceSpan { line: 1, column: 2 }),
          Token::Eof,
        ],
      ),
      (
        "%",
        vec![
          Token::Percent(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "%%",
        vec![
          Token::PercentPercent(SourceSpan { line: 1, column: 2 }),
          Token::Eof,
        ],
      ),
      (
        "=",
        vec![Token::Equal(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "!",
        vec![Token::Bang(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "<",
        vec![
          Token::LessThan(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "<=",
        vec![
          Token::LessThanOrEqual(SourceSpan { line: 1, column: 2 }),
          Token::Eof,
        ],
      ),
      (
        ">",
        vec![
          Token::GreaterThan(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        ">=",
        vec![
          Token::GreaterThanOrEqual(SourceSpan { line: 1, column: 2 }),
          Token::Eof,
        ],
      ),
      (
        "&",
        vec![
          Token::Ampersand(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        "|",
        vec![Token::Pipe(SourceSpan { line: 1, column: 1 }), Token::Eof],
      ),
      (
        "(",
        vec![
          Token::LeftParen(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      (
        ")",
        vec![
          Token::RightParen(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ],
      ),
      ("", vec![Token::Eof]),
    ];

    for (input, expected_output) in test_cases {
      let actual = LexLuthor::new(input.to_owned()).lex();

      assert_eq!(Ok(expected_output), actual);
    }
  }

  #[test]
  fn errors_on_unknown_tokens() {
    let test_cases = vec![
      (
        "?",
        vec![LexLuthorError::UnexpectedCharacter {
          source_span: SourceSpan { line: 1, column: 1 },
          message: "unexpected character ?".to_owned(),
        }],
      ),
      (
        "+-=/    ?",
        vec![LexLuthorError::UnexpectedCharacter {
          source_span: SourceSpan { line: 1, column: 9 },
          message: "unexpected character ?".to_owned(),
        }],
      ),
    ];

    for (input, expected_error) in test_cases {
      let actual = LexLuthor::new(input.to_owned()).lex();

      assert_eq!(Err(expected_error), actual);
    }
  }

  #[test]
  fn recognizes_lines_and_columns() {
    let test_cases = vec![
      (
        "+",
        Ok(vec![
          Token::Plus(SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ]),
      ),
      (
        "\n+",
        Ok(vec![
          Token::Plus(SourceSpan { line: 2, column: 1 }),
          Token::Eof,
        ]),
      ),
      (
        "+\n-",
        Ok(vec![
          Token::Plus(SourceSpan { line: 1, column: 1 }),
          Token::Minus(SourceSpan { line: 2, column: 1 }),
          Token::Eof,
        ]),
      ),
      (
        "\n\n\n     !",
        Ok(vec![
          Token::Bang(SourceSpan { line: 4, column: 6 }),
          Token::Eof,
        ]),
      ),
    ];

    for (input, expected) in test_cases {
      let actual = LexLuthor::new(input.to_owned()).lex();

      assert_eq!(expected, actual);
    }
  }

  #[test]
  fn keywords() {
    let test_cases = vec![
      (
        "program",
        vec![
          Token::Program(SourceSpan { line: 1, column: 7 }),
          Token::Eof,
        ],
      ),
      (
        "define",
        vec![Token::Define(SourceSpan { line: 1, column: 6 }), Token::Eof],
      ),
      (
        "not",
        vec![Token::Not(SourceSpan { line: 1, column: 3 }), Token::Eof],
      ),
      (
        "variable",
        vec![
          Token::Variable(SourceSpan { line: 1, column: 8 }),
          Token::Eof,
        ],
      ),
      (
        "is",
        vec![Token::Is(SourceSpan { line: 1, column: 2 }), Token::Eof],
      ),
      (
        "natural",
        vec![
          Token::Natural(SourceSpan { line: 1, column: 7 }),
          Token::Eof,
        ],
      ),
      (
        "real",
        vec![Token::Real(SourceSpan { line: 1, column: 4 }), Token::Eof],
      ),
      (
        "char",
        vec![Token::Char(SourceSpan { line: 1, column: 4 }), Token::Eof],
      ),
      (
        "boolean",
        vec![
          Token::Boolean(SourceSpan { line: 1, column: 7 }),
          Token::Eof,
        ],
      ),
      (
        "execute",
        vec![
          Token::Execute(SourceSpan { line: 1, column: 7 }),
          Token::Eof,
        ],
      ),
      (
        "set",
        vec![Token::Set(SourceSpan { line: 1, column: 3 }), Token::Eof],
      ),
      (
        "get",
        vec![Token::Get(SourceSpan { line: 1, column: 3 }), Token::Eof],
      ),
      (
        "to",
        vec![Token::To(SourceSpan { line: 1, column: 2 }), Token::Eof],
      ),
      (
        "put",
        vec![Token::Put(SourceSpan { line: 1, column: 3 }), Token::Eof],
      ),
      (
        "loop",
        vec![Token::Loop(SourceSpan { line: 1, column: 4 }), Token::Eof],
      ),
      (
        "while",
        vec![Token::While(SourceSpan { line: 1, column: 5 }), Token::Eof],
      ),
      (
        "do",
        vec![Token::Do(SourceSpan { line: 1, column: 2 }), Token::Eof],
      ),
      (
        "true",
        vec![Token::True(SourceSpan { line: 1, column: 4 }), Token::Eof],
      ),
      (
        "false",
        vec![Token::False(SourceSpan { line: 1, column: 5 }), Token::Eof],
      ),
    ];

    for (input, expected) in test_cases {
      let actual = LexLuthor::new(input.to_owned()).lex();

      assert_eq!(Ok(expected), actual);
    }
  }

  #[test]
  fn identifiers() {
    let test_cases = vec![
      (
        "x",
        Ok(vec![
          Token::Identifier("x".to_owned(), SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ]),
      ),
      (
        "_x",
        Ok(vec![
          Token::Identifier("_x".to_owned(), SourceSpan { line: 1, column: 2 }),
          Token::Eof,
        ]),
      ),
      (
        "_",
        Ok(vec![
          Token::Identifier("_".to_owned(), SourceSpan { line: 1, column: 1 }),
          Token::Eof,
        ]),
      ),
      (
        "x__",
        Err(vec![LexLuthorError::InvalidIdentifier {
          source_span: SourceSpan { line: 1, column: 3 },
          message: "x__ is not a valid identifier, _ must be followed by a letter".to_owned(),
        }]),
      ),
      (
        "x2",
        Err(vec![LexLuthorError::InvalidIdentifier {
          source_span: SourceSpan { line: 1, column: 2 },
          message: "x2 is not a valid identifier, 2 must be followed by a letter".to_owned(),
        }]),
      ),
      (
        "x2y_z2w",
        Ok(vec![
          Token::Identifier("x2y_z2w".to_owned(), SourceSpan { line: 1, column: 7 }),
          Token::Eof,
        ]),
      ),
      (
        "__",
        Err(vec![LexLuthorError::InvalidIdentifier {
          source_span: SourceSpan { line: 1, column: 2 },
          message: "__ is not a valid identifier, _ must be followed by a letter".to_owned(),
        }]),
      ),
      (
        "__variable_name",
        Err(vec![LexLuthorError::InvalidIdentifier {
          source_span: SourceSpan {
            line: 1,
            column: 15,
          },
          message: "__variable_name is not a valid identifier, _ must be followed by a letter"
            .to_owned(),
        }]),
      ),
    ];

    for (input, expected) in test_cases {
      let actual = LexLuthor::new(input.to_owned()).lex();

      assert_eq!(expected, actual);
    }
  }
}
