use crate::tokens::Token;

#[derive( Debug, PartialEq, Copy, Clone)]
pub enum VariableType{
    U8,
    U16,
    U32,
    U64,
}

impl crate::ast::AstNode for VariableType {
    fn build_from_tokens<'src, I: Iterator<Item = crate::tokens::Token<'src>>>( 
        token_iterator: &mut std::iter::Peekable<I> 
      ) -> Result<Self, super::AstNodeBuildError> 
        where Self: Sized {
        
          while let Some( token ) = token_iterator.next() {
            match token {
              Token::Whitespace( _ ) => { },
              Token::Word( type_name ) => {
                match type_name {
                  "u8" => { return Ok( Self::U8 ) },
                  "u16" => { return Ok( Self::U16 ) },
                  "u32" => { return Ok( Self::U32 ) },
                  "u64" => { return Ok( Self::U64 ) },
                  _ => { return Err( super::AstNodeBuildError::UnexpectedToken( format!("Got an invalid type: {:?}. Valids are [u8, u16, u32, u64]", token) ) ) }
                }
              },
              _ => { return Err( super::AstNodeBuildError::UnexpectedToken( format!("Got an unexpected token: {:?}", token) ))}
            }
          }
          Err( super::AstNodeBuildError::UnexpectedEOS )
    }
}

#[cfg(test)]
#[test]
fn test_variable_type_u8(){
  use super::AstNode;
  let u8_t = vec![Token::Word("u8")];
  let variable_type = VariableType::build_from_tokens( &mut u8_t.into_iter().peekable() ).unwrap();
  assert_eq!(  VariableType::U8,  variable_type );
}

#[cfg(test)]
#[test]
fn test_variable_type_u16(){
  use super::AstNode;
  let u16_t = vec![Token::Word("u16")];
  let variable_type = VariableType::build_from_tokens( &mut u16_t.into_iter().peekable() ).unwrap();
  assert_eq!(  VariableType::U16,  variable_type );
}

#[cfg(test)]
#[test]
fn test_variable_type_u32(){
  use super::AstNode;
  let u32_t = vec![Token::Word("u32")];
  let variable_type = VariableType::build_from_tokens( &mut u32_t.into_iter().peekable() ).unwrap();
  assert_eq!(  VariableType::U32,  variable_type );
}

#[cfg(test)]
#[test]
fn test_variable_type_u64(){
  use super::AstNode;
  let u64_t = vec![Token::Word("u64")];
  let variable_type = VariableType::build_from_tokens( &mut u64_t.into_iter().peekable() ).unwrap();
  assert_eq!(  VariableType::U64,  variable_type );
}

#[cfg(test)]
#[test]
fn test_variable_type_invalid(){
  use crate::ast::{AstNode, AstNodeBuildError};
  let f8_t = vec![Token::Word("f8")];
  let variable_error = VariableType::build_from_tokens( &mut f8_t.into_iter().peekable() ).unwrap_err();
  assert!( matches!( variable_error, AstNodeBuildError::UnexpectedToken( _placeholder )) )
}

#[cfg(test)]
#[test]
fn test_leave_space(){
  /// This test ensure the variabe consumes only its tokens and no more
  use crate::ast::AstNode; 
  let mut u8_t = vec![Token::Word("u8"), Token::Whitespace(" ")].into_iter().peekable();
  let _variable_type = VariableType::build_from_tokens( &mut u8_t ).unwrap();
  assert_eq!( Token::Whitespace(" "), u8_t.next().unwrap() )
}