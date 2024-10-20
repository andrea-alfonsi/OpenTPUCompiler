use crate::tokens::Token;

#[derive(Debug, PartialEq)]
pub enum Statement<'src> {
    RuleDef(crate::ast::ruledef::RuleDef<'src>),
    Instruction(crate::ast::instruction::AsmInstruction<'src>),
}

impl<'src> Statement<'src> {
    pub fn build_from_tokens<T: Iterator<Item = Token<'src>>>(
        s: &mut std::iter::Peekable<T>,
    ) -> Result<Self, &'static str> {
        while let Some( token ) = s.peek() {
            match token {
                Token::Hash =>{
                    s.next();
                    if let Some( token ) = s.peek() {
                        match token {
                            Token::Word( "ruledef" ) => { 
                                s.next();
                                while let Some( token ) = s.peek() {
                                    match token {
                                        Token::LineBreak => { s.next(); }
                                        Token::Whitespace( _ ) => { s.next(); },
                                        Token::BraceOpen => {
                                            s.next();
                                            let ruledef = crate::ast::ruledef::RuleDef::build_from_tokens( s );
                                            match ruledef {
                                                Err( e ) => return Err( e ),
                                                Ok( rd ) => return Ok(Self::RuleDef(rd))
                                            }
                                        },
                                        _ => return Err("Unexpected token. Expected are `{` only ")
                                    }
                                }
                                return Err("Unexpected EOF")
                                
                            },
                            _ => return Err("Unexpected token. Extected `ruledef` after `#`")
                        }
                    }
                },
                Token::Whitespace( _ ) => { s.next(); },
                Token::LineBreak => { s.next(); },
                Token::Word( _ ) => {
                    let instruction = crate::ast::instruction::AsmInstruction::build_from_tokens(s);
                    match instruction {
                        Err( e ) => return Err( e ),
                        Ok( i ) => return Ok( Self::Instruction( i ))
                    }
                },
                Token::Comment( _ ) => { s.next(); }
                _ => return Err("Unexpected token")
            }
        }
        Err("Unexpected EOF while matching Statement directive")
    }
}