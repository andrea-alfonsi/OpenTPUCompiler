#[derive(Debug, PartialEq)]
pub struct RuleDef<'src> {
    pub rules: Vec<crate::ast::rule::Rule<'src>>,
}

impl<'src> RuleDef<'src> {
    pub fn build_from_tokens<T: Iterator<Item = crate::tokens::Token<'src>>>(
        s: &mut std::iter::Peekable<T>,
    ) -> Result<Self, &'static str> {
        let mut rules = Vec::new();
        while let Some(token) = s.peek() {
            match token {
                crate::tokens::Token::Error => return Err("Cannot build RuleDef because got an error token"),
                crate::tokens::Token::Whitespace( _ ) => { s.next();}
                crate::tokens::Token::Comment(_) => { s.next(); }
                crate::tokens::Token::BraceClose => { s.next(); return Ok(Self { rules } ) },
                crate::tokens::Token::LineBreak => { s.next(); },
                _ => {
                    let rule = crate::ast::rule::Rule::build_from_tokens(s);
                    match rule {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(r) => {
                            rules.push(r);
                        }
                    }
                }
            }
        }
        Err("Unexpected EOF while matching RuleDef directive")
    }
}