use crate::ast::{instruction::AsmInstruction, rule::Rule};


#[derive(Debug, Clone)]
pub struct Context<'src> {
  rules: Vec<Rule<'src>>
}

impl<'src> Context<'src> {
  pub fn new() -> Self { Self { rules: Vec::new() }}

  pub fn add_rule( &mut self, rule: Rule<'src> ){
    self.rules.push( rule );
  }

  pub fn try_assemble( &self, instruction: &AsmInstruction ) ->  Result<Vec<u8>, String> {
    let mut best_rule: Option<(&Rule<'src>, u32)> = None;
    for rule in &self.rules {
      match rule.try_assemble( instruction ) {
        Ok( res ) =>  return Ok( res ),
        Err( score ) => {
          match best_rule {
            Some( old )  => {
              if score > old.1 {
                best_rule = Some( (rule, score) )
              }
            }
            None => {
              best_rule = Some(( rule, score ));
            }
          }
        }
      }
    };
    Err( format!("No rule found. Maybe you were looking for: {}", best_rule.unwrap().0.get_hint()) )
  }

  pub fn try_disassemble( &self, bytes: &Vec<u8>) -> Result<String, &'static str> {
    for rule in &self.rules {
      if let Some( result ) = rule.try_disassemble( bytes ) {
        return Ok( result )
      }
    }
    Err("Unable to disassemble")
  }
}