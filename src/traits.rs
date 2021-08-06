use crate::token_type::TokenType;
use crate::token::Token;

pub trait Lexer {
  fn scan_tokens(&mut self) -> Vec<Token>;
  fn is_at_end(&self) -> bool;
  fn scan_token(&mut self);
  fn add_token(&mut self, token: TokenType);
  fn add_token_literal(&mut self, token: TokenType, literal: &str);
  fn advance(&mut self) -> char;
  fn advance_if_next(&mut self, expected: char) -> bool;
  fn peek(&self) -> char;
}

pub trait Serialise {
  fn to_string(&self) -> &str;
}