pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

  // One or two character tokens.
  BANG, BangEqual,
  EQUAL, EqualEqual,
  GREATER, GreaterEqual,
  LESS, LessEqual,

  // Literals.
  Identifier, STRING, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}