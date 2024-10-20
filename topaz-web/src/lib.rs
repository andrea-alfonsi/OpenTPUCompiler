use topaz::{ast::statement::Statement, context, tokenizer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct WasmOptions {

}


#[wasm_bindgen]
pub fn run_topaz( src: &str, _options: Option<WasmOptions>) -> Vec<u8> {
    let mut tokenizer = tokenizer::Tokenizer::new( src ).peekable();
    let mut context = context::Context::new();
    let mut result = Vec::new();
    while let Ok(stmt) = Statement::build_from_tokens( &mut tokenizer ){
        match stmt {
            Statement::RuleDef(rule_def) => {
                for rule in rule_def.rules{
                    context.add_rule( rule );
                }
            },
            Statement::Instruction(asm_instruction) => {
                let print = context.try_assemble( &asm_instruction ).unwrap();
                result.extend( print );
            },
        }
    };
    result

}