use crate::source_code::SourceSpan;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct LexLuthorError {
  source_span: SourceSpan,
  message: String,
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

  fn next_character_is(&self, expected_character: char) -> bool {
    match self.source_code.chars().nth(self.position) {
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
      '!' => Token::Not(self.current_source_span()),
      '(' => Token::LeftParen(self.current_source_span()),
      ')' => Token::RightParen(self.current_source_span()),
      character => {
        self.read_character();
        return Err(LexLuthorError {
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
        vec![Token::Not(SourceSpan { line: 1, column: 1 }), Token::Eof],
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
        vec![LexLuthorError {
          source_span: SourceSpan { line: 1, column: 1 },
          message: "unexpected character ?".to_owned(),
        }],
      ),
      (
        "+-=/    ?",
        vec![LexLuthorError {
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
}
