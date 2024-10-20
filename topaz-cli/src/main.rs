use core::str;
use std::{io::Write, path::PathBuf};
use clap::{Parser, ValueEnum};
use topaz::{ast::statement::Statement, context, tokenizer};

#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum OutputFormat {
  BINARY,
  TEXT
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, action = clap::ArgAction::Count, help="Increase verbose level")]
  verbose: u8,
  #[arg(short, long, value_name="FORMAT", help="Set output output format")]
  format: Option<OutputFormat>,
  #[arg(short, long, value_name="INCLUDE_FILES", help="Include files before processing all the inputs")]
  include: Vec<PathBuf>,
  files: Vec<PathBuf>,
}

fn main(){
  let args = Args::parse();
  let mut input = String::new();
  for include_file in args.include{
    let content = std::fs::read( &include_file ).expect( &format!("Cannot read file '{}'. Exiting", include_file.display()) );
    input.push_str( str::from_utf8( &content).expect( &format!("Encountered not utf-8 cahracters in '{}'", include_file.display()) ));
    input.push_str("\n");
  }
  for file in args.files {
    let content = std::fs::read( &file ).expect( &format!("Cannot read file '{}'. Exiting", file.display()) );
    input.push_str( str::from_utf8( &content).expect( &format!("Encountered not utf-8 cahracters in '{}'", file.display()) ));
    input.push_str("\n");
  }
  let mut tokenizer = tokenizer::Tokenizer::new( &input ).peekable();
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
          result.push( print );
        },
    }
  }

  if let Some( format ) = args.format {
    if format == OutputFormat::TEXT {
      println!("{}", result.into_iter().flatten().map(|num|{ format!("{:02}", num) }).collect::<String>() );
    }
    else {
      std::io::stdout().write_all( &result.into_iter().flatten().collect::<Vec<u8>>() ).expect("Cannot write the output");
    }
  } else {
    std::io::stdout().write_all( &result.into_iter().flatten().collect::<Vec<u8>>() ).expect("Cannot write the output");
  }
}