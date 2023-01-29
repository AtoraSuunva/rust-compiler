use super::tokens::{error_type::ErrorType, location::Location, token::Token, token_type::Type};

pub struct LexerScanner {
    content: String,
    line: usize,
    column: usize,
    current_byte: usize,
}

impl LexerScanner {
    pub fn new(content: &str) -> LexerScanner {
        LexerScanner {
            content: content.to_owned(),
            line: 1,
            column: 1,
            current_byte: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut lexeme = String::new();
        let mut chars = self.content[self.current_byte..].char_indices();
        let last_byte = self.current_byte + 1;

        loop {
            let start_line = self.line;
            let start_column = self.column;

            match chars.next() {
                Some((i, b)) => {
                    self.current_byte = last_byte + i;
                    self.column += 1;

                    if b.is_ascii_whitespace() {
                        // Skip whitespace
                        if b == '\n' {
                            self.line += 1;
                            self.column = 1;
                        }
                        continue;
                    } else if b.is_ascii_alphabetic() {
                        // Id
                        lexeme.push(b);
                        for (i, b) in chars.by_ref() {
                            if b.is_ascii_alphanumeric() || b == '_' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                            } else {
                                break;
                            }
                        }
                        // Return Id (or keyword)
                        return Some(Token::new(
                            Type::from_alphanum(&lexeme),
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b.is_ascii_digit() {
                        // Does it start with a 0?
                        let leading_zero = b == '0';
                        // The size of the "whole" number, left of the .
                        let mut whole_size = 1;
                        let mut is_float = false;

                        // Integer or Float
                        lexeme.push(b);
                        for (i, b) in chars.by_ref() {
                            if b.is_ascii_digit() {
                                whole_size += 1;
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                            } else if b == '.' {
                                is_float = true;
                                self.current_byte = last_byte + i;
                                lexeme.push('.');
                                break;
                            } else {
                                break;
                            }
                        }

                        if is_float {
                            let mut has_exponent = false;
                            // Does the fractional part end with a 0?
                            let mut trailing_zero = false;
                            // The size of the fractional part, right of the .
                            let mut fractional_size = 0;

                            for (i, b) in chars.by_ref() {
                                if b.is_ascii_digit() {
                                    trailing_zero = b == '0';
                                    fractional_size += 1;
                                    self.current_byte = last_byte + i;
                                    lexeme.push(b);
                                    self.column += 1;
                                } else if b == 'e' {
                                    has_exponent = true;
                                    self.current_byte = last_byte + i;
                                    lexeme.push('e');
                                    break;
                                } else {
                                    break;
                                }
                            }

                            let mut leading_exponent_zero = false;
                            let mut exponent_size = 0;

                            if has_exponent {
                                for (i, b) in chars.by_ref() {
                                    if b.is_ascii_digit() || b == '+' || b == '-' {
                                        if b.is_ascii_digit() {
                                            if exponent_size == 0 {
                                                leading_exponent_zero = b == '0';
                                            }
                                            exponent_size += 1;
                                        }
                                        self.current_byte = last_byte + i;
                                        lexeme.push(b);
                                        self.column += 1;
                                    } else {
                                        break;
                                    }
                                }
                            }

                            if leading_zero && whole_size > 1 {
                                return Some(Token::new(
                                    Type::Invalid(ErrorType::LeadingZero()),
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }

                            if trailing_zero && fractional_size > 1 {
                                return Some(Token::new(
                                    Type::Invalid(ErrorType::TrailingZero()),
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }

                            if leading_exponent_zero && exponent_size > 1 {
                                return Some(Token::new(
                                    Type::Invalid(ErrorType::LeadingZero()),
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }

                            // Return Float
                            return Some(Token::new(
                                Type::FloatNum(lexeme.parse::<f64>().unwrap()),
                                lexeme,
                                Location::new(start_line, start_column),
                            ));
                        }

                        if leading_zero && whole_size > 1 {
                            return Some(Token::new(
                                Type::Invalid(ErrorType::LeadingZero()),
                                lexeme,
                                Location::new(start_line, start_column),
                            ));
                        }

                        // Return Integer
                        return Some(Token::new(
                            Type::IntNum(lexeme.parse::<isize>().unwrap()),
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '=' {
                        lexeme.push(b);

                        if let Some((i, b)) = chars.next() {
                            if b == '=' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::Eq,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            } else if b == '>' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::ReturnType,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }
                        }

                        return Some(Token::new(
                            Type::Assign,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '+' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Plus,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '<' {
                        lexeme.push(b);

                        if let Some((i, b)) = chars.next() {
                            if b == '>' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::NotEq,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            } else if b == '=' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::LEq,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }
                        }

                        return Some(Token::new(
                            Type::Lt,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '-' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Minus,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '>' {
                        lexeme.push(b);

                        if let Some((i, b)) = chars.next() {
                            if b == '=' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::GEq,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }
                        }

                        return Some(Token::new(
                            Type::Gt,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '*' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Mult,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '/' {
                        lexeme.push(b);

                        if let Some((i, b)) = chars.next() {
                            if b == '/' {
                                // Inline comment
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;

                                // Consume the rest of the line
                                for (i, b) in chars.by_ref() {
                                    if b != '\n' {
                                        self.current_byte = last_byte + i;
                                        if b != '\r' {
                                            lexeme.push(b);
                                            self.column += 1;
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                return Some(Token::new(
                                    Type::InlineCmt,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            } else if b == '*' {
                                // Block comment
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;

                                // Consume the rest of the block
                                let mut layers_deep = 1;
                                let mut found_star = false;
                                let mut found_slash = false;
                                let mut comment_ended = false;

                                for (i, b) in chars.by_ref() {
                                    self.current_byte = last_byte + i;
                                    lexeme.push(b);
                                    self.column += 1;

                                    if found_star && b == '/' {
                                        layers_deep -= 1;
                                        if layers_deep == 0 {
                                            comment_ended = true;
                                            break;
                                        };
                                    }

                                    if found_slash && b == '*' {
                                        layers_deep += 1;
                                    }

                                    if b == '\n' {
                                        self.line += 1;
                                        self.column = 1;
                                    } else {
                                        self.column += 1;
                                    }

                                    found_star = b == '*';
                                    found_slash = b == '/';
                                }

                                if !comment_ended {
                                    return Some(Token::new(
                                        Type::Invalid(ErrorType::UnclosedBlockCmt()),
                                        lexeme,
                                        Location::new(start_line, start_column),
                                    ));
                                }

                                return Some(Token::new(
                                    Type::BlockCmt,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }
                        }

                        return Some(Token::new(
                            Type::Div,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '(' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::OpenPar,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == ')' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::ClosePar,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '{' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::OpenCubr,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '}' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::CloseCubr,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '[' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::OpenSqbr,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == ']' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::CloseSqbr,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == ';' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Semi,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == ',' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Comma,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == '.' {
                        lexeme.push(b);
                        return Some(Token::new(
                            Type::Dot,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    } else if b == ':' {
                        lexeme.push(b);

                        if let Some((i, b)) = chars.next() {
                            if b == ':' {
                                self.current_byte = last_byte + i;
                                lexeme.push(b);
                                self.column += 1;
                                return Some(Token::new(
                                    Type::ScopeOp,
                                    lexeme,
                                    Location::new(start_line, start_column),
                                ));
                            }
                        }

                        return Some(Token::new(
                            Type::Colon,
                            lexeme,
                            Location::new(start_line, start_column),
                        ));
                    }

                    // We didn't match anything
                    lexeme.push(b);
                    return Some(Token::new(
                        Type::Invalid(ErrorType::InvalidChar()),
                        lexeme,
                        Location::new(start_line, start_column),
                    ));
                }
                // Reached EOF
                None => return None,
            }
        }
    }
}

impl Iterator for LexerScanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
