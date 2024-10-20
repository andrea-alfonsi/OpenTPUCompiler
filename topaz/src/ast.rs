pub mod instruction;
pub mod rule;
pub mod ruledef;
pub mod statement;
pub mod variable;
pub mod variable_type;

#[derive(Clone, Debug)]
pub enum AstNodeBuildError {
  /// The token stream yield a Token::Error maning that a non valid character was in the input
  TokenError( String ),
  /// The  token stream yield a Token which was not in the correct position in order to buildthe desired node
  UnexpectedToken( String ),
  /// EndOfStream was encountered before finish building the node
  UnexpectedEOS
}

/// This trait is used by all the elements in the ast to be crated from the token stream
/// The element should consume all the tokens that belongs to him
pub trait AstNode {
  fn build_from_tokens<'src, I: Iterator<Item = crate::tokens::Token<'src>>>( 
    token_iterator: &mut std::iter::Peekable<I> 
  ) -> Result<Self, AstNodeBuildError> 
    where Self: Sized;
}


#[cfg(test)]
#[test]
fn simple_instruction(){
    let tokenier =  crate::tokenizer::Tokenizer::new( "add 6 8\n" );
    let stmt  = crate::ast::statement::Statement::build_from_tokens( &mut tokenier.into_iter().peekable() ).unwrap();
    assert_eq!( stmt, crate::ast::statement::Statement::Instruction( crate::ast::instruction::AsmInstruction { instructions: vec![crate::tokens::Token::Word("add"), crate::tokens::Token::Number("6"), crate::tokens::Token::Number("8")] }))
}

#[cfg(test)]
#[test]
fn ruledef_empty(){
    let tokenier =  crate::tokenizer::Tokenizer::new( "#ruledef{\n}" );
    let stmt  = crate::ast::statement::Statement::build_from_tokens( &mut tokenier.into_iter().peekable() ).unwrap();
    assert_eq!( stmt, crate::ast::statement::Statement::RuleDef( crate::ast::ruledef::RuleDef { rules: vec![] }))
}

#[cfg(test)]
#[test]
fn ruledef_spaces(){
    let tokenier =  crate::tokenizer::Tokenizer::new( "#ruledef{\n \n}" );
    let stmt  = crate::ast::statement::Statement::build_from_tokens( &mut tokenier.into_iter().peekable() ).unwrap();
    assert_eq!( stmt, crate::ast::statement::Statement::RuleDef( crate::ast::ruledef::RuleDef { rules: vec![] }))
}

#[cfg(test)]
#[test]
fn ruledef_simple_rule(){
    use crate::ast::{rule::{RuleLeftItem, RuleNumber, RuleRightItem}, variable::Variable, variable_type::VariableType};

    let tokenier =  crate::tokenizer::Tokenizer::new( "#ruledef{\nadd {n1: u8} {n2: u8} => 11 {n1} {n2}\n}" );
    let stmt  = crate::ast::statement::Statement::build_from_tokens( &mut tokenier.into_iter().peekable() ).unwrap();
    assert_eq!( stmt, crate::ast::statement::Statement::RuleDef( crate::ast::ruledef::RuleDef { rules: 
        vec![ crate::ast::rule::Rule { 
            left_items: vec![ 
                RuleLeftItem::Word("add"), 
                RuleLeftItem::Variable( Variable { name: "n1", v_type: VariableType::U8 } ),
                RuleLeftItem::Variable( Variable { name: "n2", v_type: VariableType::U8 } ) ], 
            right_items: vec![
                RuleRightItem::Number( RuleNumber { value: "11" }),
                RuleRightItem::Variable( Variable { v_type: VariableType::U8, name: "n1" }),
                RuleRightItem::Variable( Variable { v_type: VariableType::U8, name: "n2" })
            ] } ] }))
}