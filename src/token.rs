use std::u16;
use crate::token_type::TokenType;
use crate::traits::Serialise;

pub struct Token {
  pub token: TokenType,
  pub lexeme: String,
  pub literal: String,
  pub line: u16
}
 
impl Serialise for Token {
  fn to_string(&self) -> &str {
    // return type + " " + lexeme + " " + literal;
    return "";
  }
}