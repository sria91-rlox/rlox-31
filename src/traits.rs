use crate::token_type::TokenType;
use crate::token::Token;

pub trait Lexer {
  fn scan_tokens(&self) -> Vec<Token>;
  fn is_at_end(&self) -> bool;
  fn scan_token(&self) -> Token;
  fn get_next_char(&self) -> char;
  fn add_token(&self, token: TokenType);
  fn add_token_literal(&self, token: TokenType, literal: &str);
}

pub trait Serialise {
  fn to_string(&self) -> &str;
}