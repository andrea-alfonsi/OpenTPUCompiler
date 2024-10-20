use crate::tokens::Token;

use super::{variable_type::VariableType, AstNode};


#[derive( Debug, PartialEq, Copy, Clone)]
pub struct Variable<'src> {
    pub v_type: VariableType,
    pub name: &'src str
}

impl<'src> Variable<'src>{
    pub fn build_from_tokens<T: Iterator<Item=Token<'src>>>(s: &mut std::iter::Peekable<T>) -> Result<Self, &'static str> {
        while let Some( token ) = s.peek() {
            match token {
                Token::Error => return Err("Cannot build variable because got an error token"),
                Token::Comment( _ ) => {},
                Token::Whitespace( _ ) => {}, 
                Token::Word( n ) => {
                    let name = *n;
                    s.next();
                    while let Some( token ) = s.peek() {
                        match token {
                            Token::Whitespace( _ ) => {},
                            Token::Colon => {
                                s.next();
                                let v_type = VariableType::build_from_tokens( s ).unwrap();
                                return Ok( Self{ v_type, name })
                            }
                            _ => { return Err("Unexpected token")}
                        }
                        s.next();
                    }
                },
                _ => { return Err("Unexpected token")}
            };
            s.next();
        }
        Err("Unexpected EOF while metching Variable directive")
    }
}