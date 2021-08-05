
use crate::traits::Lexer;
use crate::error_reporter::error;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
  source: &'static str,
  tokens: Vec<Token>,
  start: u32,
  current: u32,
  line: u32
}

impl Lexer for Scanner {
  fn scan_tokens(&self) -> Vec<Token> {
    let mut tokens = vec![]; 
    while !self.is_at_end() {

      // tokens.push(value: T)
    } 
    tokens.push(Token {
      token: TokenType::Eof,
      lexeme: "some part of speech".to_string(),
      line: 1,
      literal: "super literal".to_string()
    });
    return tokens;
  }

  fn scan_token(&mut self) {
    let character = self.get_next_char();
    match character {
      '(' => self.add_token(TokenType::LeftParen),
      ')' => self.add_token(TokenType::RightParen),
      '{' => self.add_token(TokenType::LeftBrace),
      '}' => self.add_token(TokenType::RightBrace),
      ',' => self.add_token(TokenType::Comma),
      '.' => self.add_token(TokenType::Dot),
      '-' => self.add_token(TokenType::Minus),
      '+' => self.add_token(TokenType::Plus),
      ';' => self.add_token(TokenType::SemiColon),
      '*' => self.add_token(TokenType::Star),
      _ => error(self.line, "Unexpected character")
    }
  }
  
  fn add_token(&mut self, token: TokenType) {
    self.add_token_literal(token, "");
  }

  fn add_token_literal(&mut self, token: TokenType, literal: &str) {
    let lexeme: String = self.source
      .chars()
      .skip(self.start as usize)
      .take(self.current as usize)
      .collect();
    self.tokens.push(Token {
      token,
      lexeme,
      line: 1,
      literal: literal.to_string()
    });
  }

  fn get_next_char(&mut self) -> char {
    self.current += 1;
    let next = self.source.as_bytes()[self.current as usize];
    return next as char;
  }

  fn is_at_end(&self) -> bool {
    return self.current >= self.source.len() as u32;
  }
}
