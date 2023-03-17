#[derive(Debug)]
pub enum LexerError {
  NumberError { number: String },
  UnknownSymbol { symbol: char },
  UnknownStringEscape { symbol: char },
}

#[derive(Debug)]
pub enum BracketKind {
  Round,
  Square,
  Curly,
}

#[derive(Debug)]
pub enum BracketOrientation {
  Open,
  Close,
}

#[derive(Debug)]
pub enum SeparatorKind {
  Dot,
  Comma,
  Colon,
  Semicolon,
}

#[derive(Debug)]
pub enum OperatorKind {
  Plus,
  Minus,
  Multiply,
  Divide,
  And,
  Or,
  Equal,
  Less,
  More,
  Wave,
  Not,
  Backslash,
  Underscore,
  Roof,
}

#[derive(Debug)]
pub enum NumberKind {
  Integer,
  Float,
}

#[derive(Debug)]
pub enum Token {
  EOF,
  Bracket {
    raw: char,
    kind: BracketKind,
    orientation: BracketOrientation,
  },
  Separator {
    raw: char,
    kind: SeparatorKind,
  },
  Operator {
    raw: char,
    kind: OperatorKind,
  },
  Identifier {
    content: String,
  },
  String {
    raw: String,
  },
  Number {
    raw: String,
    kind: NumberKind,
  }
}

pub struct Lexer<'a> {
  pub cur_line: usize,
  pub cur_col: usize,

  pub codepoint_offset: usize,

  chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(chars: &'a str) -> Lexer<'a> {
    Lexer {
      cur_line: 1,
      cur_col: 1,
      codepoint_offset: 0,
      chars: chars.chars().peekable(),
    }
  }

  fn match_token(&mut self, c: char) -> Result<Token, LexerError> {
    None
      .or_else(|| Self::match_bracket(c))
      .or_else(|| Self::match_separator(c))
      .or_else(|| Self::match_operator(c))
      .or_else(|| self.match_number(c))
      .or_else(|| self.match_identifier(c))
      .or_else(|| self.match_string(c))
      .unwrap_or_else(|| Err(LexerError::UnknownSymbol { symbol: c }))
  }

  fn match_bracket(c: char) -> Option<Result<Token, LexerError>> {
    let (kind, orientation) = match c {
      '(' => (BracketKind::Round, BracketOrientation::Open),
      ')' => (BracketKind::Round, BracketOrientation::Close),
      '[' => (BracketKind::Square, BracketOrientation::Open),
      ']' => (BracketKind::Square, BracketOrientation::Close),
      '{' => (BracketKind::Curly, BracketOrientation::Open),
      '}' => (BracketKind::Curly, BracketOrientation::Close),
      _ => return Option::None,
    };

    Some(Ok(Token::Bracket {
      raw: c,
      kind,
      orientation,
    }))
  }

  fn match_separator(c: char) -> Option<Result<Token, LexerError>> {
    let kind = match c {
      '.' => SeparatorKind::Dot,
      ',' => SeparatorKind::Comma,
      ':' => SeparatorKind::Colon,
      ';' => SeparatorKind::Semicolon,
      _ => return None,
    };

    Some(Ok(Token::Separator { raw: c, kind }))
  }

  fn match_operator(c: char) -> Option<Result<Token, LexerError>> {
    let kind = match c {
      '+' => OperatorKind::Plus,
      '-' => OperatorKind::Minus,
      '*' => OperatorKind::Multiply,
      '/' => OperatorKind::Divide,
      '&' => OperatorKind::And,
      '|' => OperatorKind::Or,
      '=' => OperatorKind::Equal,
      '<' => OperatorKind::Less,
      '>' => OperatorKind::More,
      '~' => OperatorKind::Wave,
      '!' => OperatorKind::Not,
      '\\' => OperatorKind::Backslash,
      '_' => OperatorKind::Underscore,
      '^' => OperatorKind::Roof,
      _ => return Option::None,
    };

    Some(Ok(Token::Operator { raw: c, kind }))
  }

  fn match_identifier(&mut self, c: char) -> Option<Result<Token, LexerError>> {
    let mut identifier_str = String::new();

    match c {
      'a'..='z' | 'A'..='Z' => {
        identifier_str.push(c);
      }
      _ => return None,
    }

    while let Some(c) = self.chars.peek() {
      match c {
        'a'..='z' | 'A'..='Z' => {
          identifier_str.push(*c);
        }

        _ => break,
      }

      self.consume_char();
    }

    Some(Ok(Token::Identifier {
      content: identifier_str,
    }))
  }

  fn match_string(&mut self, c: char) -> Option<Result<Token, LexerError>> {
    let mut string = String::new();

    match c {
      '"' => {}
      _ => return None,
    }

    let mut escaping = false;
    let mut error_char: Option<char> = None;

    while let Some(c) = self.chars.peek() {
      if escaping {
        escaping = false;
        let ch = match c {
          'n' => '\n',
          '\\' => '\\',
          _ => {
            error_char = Some(*c);
            *c
          }
        };
        string.push(ch);
      } else {
        match c {
          '"' => {
            self.consume_char();
            break;
          }
          '\\' => {
            escaping = true;
          }
          _ => {
            string.push(*c);
          }
        }
      }

      self.consume_char();
    }

    if let Some(c) = error_char {
      Some(Err(LexerError::UnknownStringEscape { symbol: c }))
    } else {
      Some(Ok(Token::String { raw: string }))
    }
  }

  fn match_number(&mut self, c: char) -> Option<Result<Token, LexerError>> {
    let mut kind = NumberKind::Integer;
    let mut has_error = false;

    let mut number_str = String::new();

    match c {
      '0'..='9' => {
        number_str.push(c);
      }
      _ => return None,
    }

    while let Some(c) = self.chars.peek() {
      match c {
        '0'..='9' => {
          number_str.push(*c);
        }
        '.' => {
          if let NumberKind::Float = kind {
            has_error = true;
          }
          number_str.push(*c);
          kind = NumberKind::Float;
        }
        _ => break,
      }
      self.consume_char();
    }

    if has_error {
      Some(Err(LexerError::NumberError { number: number_str }))
    } else {
      Some(Ok(Token::Number {
        raw: number_str,
        kind,
      }))
    }
  }

  fn consume_char(&mut self) -> Option<char> {
    match self.chars.next() {
      Some(c) => {
        self.cur_col += 1;
        if c == '\n' {
          self.cur_line += 1;
          self.cur_col = 1;
        }

        self.codepoint_offset += 1;

        return Some(c);
      }
      None => return None,
    }
  }

  fn consume_whitespace(&mut self) {
    while let Some(c) = self.chars.peek() {
      if !c.is_whitespace() {
        break;
      }
      self.consume_char();
    }
  }

  pub fn next_token(&mut self) -> Result<Token, LexerError> {
    self.consume_whitespace();

    if let Some(c) = self.consume_char() {
      self.match_token(c)
    } else {
      Ok(Token::EOF)
    }
  }
}
