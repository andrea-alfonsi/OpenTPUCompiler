#[derive(Debug, PartialEq)]
pub struct AsmInstruction<'src> {
    pub instructions: Vec<crate::tokens::Token<'src>>,
}
impl<'src> AsmInstruction<'src> {
    pub fn build_from_tokens<T: Iterator<Item = crate::tokens::Token<'src>>>(
        s: &mut std::iter::Peekable<T>,
    ) -> Result<Self, &'static str> {
        let mut instructions = Vec::new();
        while let Some(token) = s.peek() {
            match token {
                crate::tokens::Token::Error => return Err("Cannot build AsmInstruction because go an error token"),
                crate::tokens::Token::Whitespace( _ ) => { s.next(); },
                crate::tokens::Token::Comment(_) => { s.next(); },
                crate::tokens::Token::LineBreak => { s.next(); return Ok(Self{ instructions }) },
                _ => { instructions.push( *token ); s.next(); }
            }
        }
        if instructions.len() > 0 {
            Ok( Self{ instructions })
        } else {
            Err("Unexpected EOF while matching AsmInstruction directive")
        }
    }
}