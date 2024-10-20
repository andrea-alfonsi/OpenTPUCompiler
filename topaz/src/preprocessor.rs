use crate::tokens::Token;

pub enum Directive {
  Replace(String, Token<'static>)
}

pub struct Preprocessor {
  directives: Vec<Directive>,
}

impl Preprocessor {
  pub fn new() -> Self { Preprocessor{ directives: Vec::new() } }
  pub fn add_directie( &mut self, directive: Directive ) -> &mut Self { self.directives.push( directive ); self }
  pub fn preprocess<'src, I: Iterator<Item = Token<'src>>>( self, token_stream: I ) -> PreprocessorIterator<'src,I> {
    PreprocessorIterator::new( self, token_stream )
  }
}

pub struct PreprocessorIterator<'src, I: Iterator<Item = Token<'src> >> {
  preprocessor: Preprocessor,
  peeked: Option<Option<I::Item>>,
  token: I
}

impl<'src, I: Iterator<Item = Token<'src>>> PreprocessorIterator<'src, I> {
  pub fn new( preprocessor: Preprocessor, token_stream: I ) -> Self { Self { preprocessor, token: token_stream, peeked: None }}
}

impl<'src, I: Iterator<Item = Token<'src> >> Iterator for PreprocessorIterator<'src, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
      if let Some( item ) = self.peeked { 
        self.peeked = None;
        return item;
      }
      let next_token = self.token.next();
      match next_token {
        None => { return None }
        Some( token )  => {
          match token {
            Token::Hash => {
              if let Some( next_token ) = self.token.next() {
                match next_token {
                  Token::Word( word ) => {
                    for directive in &self.preprocessor.directives {
                      match directive {
                        Directive::Replace( from, to ) => {
                          if word == from {
                            return Some( *to )
                          }     
                        }
                      }
                    }
                  },
                  _ => {}
                }
                self.peeked = Some( Some(next_token) )
              }
              return Some(Token::Hash);
            },
            _ => { self.peeked = Some(Some(token)); }
          }
        }
      }
      Some( Token::Error )
    }
}

#[cfg(test)]
#[test]
fn test_preprocessor(){
  let code = vec![Token::Hash, Token::Word("var")];
  let mut preprocessor = Preprocessor::new();
    preprocessor.add_directie( Directive::Replace( String::from("var") , Token::Number( "5" )) );
  let mut p = preprocessor.preprocess( code.into_iter() ).into_iter();
  assert_eq!( Some(Token::Number("5")), p.next() );
  assert_eq!( None, p.next() )
}

#[cfg(test)]
#[test]
fn test_preprocessor_ignore(){
  let code = vec![Token::Hash, Token::Word("var")];
  let preprocessor = Preprocessor::new();
  let mut p = preprocessor.preprocess( code.into_iter() ).into_iter();
  assert_eq!( Some(Token::Hash), p.next() );
  assert_eq!( Some(Token::Word("var")), p.next() );
  assert_eq!( None, p.next() )
}