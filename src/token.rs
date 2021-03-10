use crate::source_code::SourceSpan;

#[derive(Debug, PartialEq)]
pub enum Token {
  LeftBrace(SourceSpan),
  RightBrace(SourceSpan),
  LeftBracket(SourceSpan),
  RightBracket(SourceSpan),
  Comma(SourceSpan),
  Plus(SourceSpan),
  Minus(SourceSpan),
  Star(SourceSpan),
  Slash(SourceSpan),
  StarStar(SourceSpan),
  Percent(SourceSpan),
  PercentPercent(SourceSpan),
  Equal(SourceSpan),
  NotEqual(SourceSpan),
  LessThan(SourceSpan),
  GreaterThan(SourceSpan),
  LessThanOrEqual(SourceSpan),
  GreaterThanOrEqual(SourceSpan),
  Ampersand(SourceSpan),
  Pipe(SourceSpan),
  Not(SourceSpan),
  LeftParen(SourceSpan),
  RightParen(SourceSpan),
  Eof,
}
