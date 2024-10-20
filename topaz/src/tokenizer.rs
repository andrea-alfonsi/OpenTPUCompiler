pub use crate::tokens::Token;

/// The tokenizer is the component that takes a string as input and returns a token stream as output
#[derive(Clone, Copy)]
pub struct Tokenizer<'src>{
    src: &'src str,
    cursor_position: usize
}

impl<'src> Tokenizer<'src>{
    /// Generate a new tokenizer from the given string
    pub fn new( src: &'src str ) -> Self {
      Tokenizer { src, cursor_position: 0 } 
    }
}

impl<'src> Iterator for Tokenizer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.src[self.cursor_position..].chars().peekable();
        match chars.next() {
            None => None,
            Some( ch ) => {
                match ch {
                    ' ' => { 
                        let start = self.cursor_position;
                        self.cursor_position += 1; 
                        while let Some( space ) = chars.peek() {
                            match space {
                                ' ' => { self.cursor_position += 1; chars.next(); }
                                _ => { break; }
                            };
                        }
                        Some( Token::Whitespace(&self.src[start..self.cursor_position]) ) 
                    },
                    '\t' => { 
                        self.cursor_position += 1; 
                        let start = self.cursor_position;
                        while let Some( space ) = chars.peek() {
                            match space {
                                '\t' => { self.cursor_position += 1; chars.next(); }
                                _ => { break; }
                            };
                        }
                        Some( Token::Whitespace(&self.src[start..self.cursor_position]) ) 
                    },
                    '\n' => { self.cursor_position += 1; Some( Token::LineBreak ) },
                    '\r' => { self.cursor_position += 1; Some( Token::LineBreak ) },
                    ',' => { self.cursor_position += 1; Some( Token::Comma ) },
                    '.' => { self.cursor_position += 1; Some( Token::Dot ) },
                    '#' => { self.cursor_position += 1; Some( Token::Hash ) },
                    '(' => { self.cursor_position += 1; Some( Token::ParenOpen ) },
                    ')' => { self.cursor_position += 1; Some( Token::ParenClose ) },
                    '[' => { self.cursor_position += 1; Some( Token::BracketOpen ) },
                    ']' => { self.cursor_position += 1; Some( Token::BracketClose ) },
                    '{' => { self.cursor_position += 1; Some( Token::BraceOpen ) },
                    '}' => { self.cursor_position += 1; Some( Token::BraceClose ) },
                    ';' => {
                        let start = self.cursor_position;
                        self.cursor_position += 1;
                        while let Some( char ) = chars.peek() {
                            match char {
                                '\n' => { break; }
                                _ => { self.cursor_position += 1;  chars.next();  }
                            }
                        }
                        Some( Token::Comment( &self.src[start..self.cursor_position] ))
                    }
                    '"' => {
                        self.cursor_position += 1;
                        let start = self.cursor_position;
                        while let Some( char ) = chars.peek() {
                            match char {
                                '\\' => { self.cursor_position += 2; chars.next(); chars.next(); }
                                '"' => { break; }
                                _ => { self.cursor_position += 1; chars.next(); }
                            }
                        }
                        if let Some( char) = chars.peek() {
                            if *char == '"' {
                                self.cursor_position += 1;
                                Some( Token::String(&self.src[start..self.cursor_position-1] ) )
                            } else {
                                Some( Token::Error )
                            }
                        } else {
                            Some( Token::Error )
                        }
                    }
                    '\'' => {
                        self.cursor_position += 1;
                        let start = self.cursor_position;
                        while &self.src[self.cursor_position..self.cursor_position+1] != "'" {
                            if &self.src[self.cursor_position..self.cursor_position+1] == "\\" {
                                self.cursor_position += 1;
                            }
                            self.cursor_position += 1;
                        }
                        Some( Token::String(&self.src[start..self.cursor_position] ) )
                    }
                    ':' => { 
                        self.cursor_position += 1; 
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            ":" => { self.cursor_position += 1; Some( Token::ColonColon ) },
                            _   => Some( Token::Colon )
                        }
                    },
                    '=' => { 
                        self.cursor_position += 1; 
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            ">" => { self.cursor_position += 1; Some( Token::HeavyArrowRight ) },
                            "=" => { self.cursor_position += 1; Some( Token::DoubleEqual ) },
                            _   => Some( Token::Equal )
                        }
                    },
                    '+' => { self.cursor_position += 1; Some( Token::Plus ) },
                    '-' => { 
                        self.cursor_position += 1;
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            ">" => { self.cursor_position += 1; Some( Token::ArrowRight ) },
                            _   => Some( Token::Minus )
                        }
                    },
                    '*' => { self.cursor_position += 1; Some( Token::Asterisk ) },
                    '/' => { self.cursor_position += 1; Some( Token::Slash ) },
                    '%' => { self.cursor_position += 1; Some( Token::Percent ) },
                    '?' => { self.cursor_position += 1; Some( Token::Question ) },
                    '!' => { self.cursor_position += 1; Some( Token::Exclamation ) },
                    '&' => { 
                        self.cursor_position += 1; 
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            "&" => { self.cursor_position += 1; Some( Token::DoubleAmpersand ) },
                            _   => Some( Token::Ampersand )
                        }
                    },
                    '|' => { 
                        self.cursor_position += 1;
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            "|" => { self.cursor_position += 1; Some( Token::DoubleVerticalBar ) },
                            _   => Some( Token::VerticalBar )
                        }
                    },
                    '^' => { self.cursor_position += 1; Some( Token::Circumflex ) },
                    '~' => { self.cursor_position += 1; Some( Token::Tilde ) },
                    '`' => { self.cursor_position += 1; Some( Token::Grave ) },
                    '@' => { self.cursor_position += 1; Some( Token::At ) },
                    '<' => { 
                        self.cursor_position += 1; 
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            "<" => { 
                                self.cursor_position += 1; 
                                match &self.src[self.cursor_position..self.cursor_position+1] {
                                    "<" => { self.cursor_position += 1; Some( Token::TripleLessThan ) },
                                    _   => Some( Token::DoubleLessThan )
                                }
                            },
                            "-" => {self.cursor_position += 1; Some( Token::ArrowLeft) }
                            "=" => {self.cursor_position += 1; Some( Token::LessThan ) }
                            _   => Some( Token::LessThan )
                        }
                    },
                    '>' => { 
                        self.cursor_position += 1; 
                        match &self.src[self.cursor_position..self.cursor_position+1] {
                            ">" => { 
                                self.cursor_position += 1; 
                                match &self.src[self.cursor_position..self.cursor_position+1] {
                                    ">" => { self.cursor_position += 1; Some( Token::TripleLessThan ) },
                                    _   => Some( Token::DoubleGreaterThan )
                                }
                            },
                            _   => Some( Token::GreaterThan )
                        }
                    },
                    _  => {
                        let start = self.cursor_position;

                        self.cursor_position += 1;
                        while let Some( c ) = chars.next() {
                            if !c.is_alphanumeric() && c != '_' && c != '.' { break; }
                            self.cursor_position += 1;
                        }
                        
                        let number_regex = regex::Regex::new( r"^([+|-]?[0-9]+(\.[0-9]+)*)|(0x[a-f0-9]+)$" ).unwrap();
                        if number_regex.is_match( &self.src[start..self.cursor_position] ) {
                            Some( Token::Number( &self.src[start..self.cursor_position] ))
                        } else {
                            Some( Token::Word( &self.src[start..self.cursor_position] ))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn tokenizer_new(){
    let _tokenizer = Tokenizer::new( "test" );
    let _tokenizer = Tokenizer::new( String::from("test").as_str() );
}

#[cfg(test)]
#[test]
fn tokenizer_numbers_dec(){
    let src = "3 3.7";
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::Number( "3" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Number( "3.7" ));
}

#[cfg(test)]
#[test]
fn tokenizer_numbers_hex(){
    let src = "0xff";
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::Number( "0xff" ));
}

#[cfg(test)]
#[test]
fn tokenizer_idents(){
    let src = "a b cde";
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "a" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "b" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "cde" ));
}


#[cfg(test)]
#[test]
fn tokenizer_idents_and_numbers(){
    let src = "a 3 c 1.7";
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "a" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Number( "3" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "c" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Number( "1.7" ));
}


#[cfg(test)]
#[test]
fn tokenizer_comments(){
    let src = "a ; 1.7\nb ";
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "a" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Whitespace( " " ) );
    assert_eq!( tokenizer.next().unwrap(), Token::Comment( "; 1.7" ));
    assert_eq!( tokenizer.next().unwrap(), Token::LineBreak );
    assert_eq!( tokenizer.next().unwrap(), Token::Word( "b" ));
}


#[cfg(test)]
#[test]
fn tokenizer_error_on_not_closed_string(){
    let src = r#""string""string"# ;
    let mut tokenizer = Tokenizer::new( src ).into_iter();
    assert_eq!( tokenizer.next().unwrap(), Token::String( "string" ));
    assert_eq!( tokenizer.next().unwrap(), Token::Error );
}