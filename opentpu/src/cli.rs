use std::{io::{self, BufRead, Write}, process::exit};
use clap::Parser;
use log::{error, info};
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;


lrlex_mod!("assembler.l");
lrpar_mod!("assembler.y");

#[derive(Parser, Debug)]
#[command(version, about = "Compiler and assembler for the OpenTPU project", long_about= None)]
struct Args {
    #[arg(short, default_value_t = 0, action = clap::ArgAction::Count )]
    verbose: u8,
    #[arg(short, default_value_t = false)]
    interactive: bool
}

fn main() {

    let args = Args::parse();
    log4rs::init_file("logging.yml", Default::default()).unwrap();

    if args.interactive {
        info!(target:"assembler", "OpenTpu Assembler started in interactive mode");
        let lexerdef = assembler_l::lexerdef();
        let stdin = io::stdin();
        loop {
            print!(">>> ");
            io::stdout().flush().ok();
            match stdin.lock().lines().next() {
                Some(Ok(ref l)) => {
                    if l.trim().is_empty() {
                        continue;
                    }
                    let ll = l.to_lowercase();
                    let lexer = lexerdef.lexer( &ll );
                    let (res, errs) = assembler_y::parse(&lexer);
                    for e in errs {
                        error!(target: "assembler", "Instruction: '{}'. Error: {}", ll, e.pp(&lexer, &assembler_y::token_epp));
                    }
                    match res {
                        Some(Ok(r)) => write_hex(r),
                        _ => eprintln!("Unable to evaluate expression.")
                    }
                }
                _ => break
            }
        }
    } else {
        let lexerdef = assembler_l::lexerdef();
        let stdin = io::stdin();
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    exit(0);
                }
                let ll = l.to_lowercase();
                let lexer = lexerdef.lexer( &ll);
                let (res, errs) = assembler_y::parse(&lexer);
                for e in errs {
                    error!(target: "assembler", "Instruction: '{}'. Error: {}", ll, e.pp(&lexer, &assembler_y::token_epp));
                }
                match res {
                    Some(Ok(r)) => write_raw(r),
                    _ => eprintln!("Unable to evaluate expression.")
                }
            }
            _ => exit( 1 )
        }
    }
}


fn write_hex( instruction: (u8, u8, (u64, u64, u64)) ){
    let res = [ instruction.0, instruction.1, 
                         *instruction.2.0.to_le_bytes().get(0).unwrap(), 
                         *instruction.2.1.to_le_bytes().get(0).unwrap(), *instruction.2.1.to_le_bytes().get(1).unwrap(), *instruction.2.1.to_le_bytes().get(2).unwrap(), *instruction.2.1.to_le_bytes().get(3).unwrap(),
                         *instruction.2.1.to_le_bytes().get(4).unwrap(), *instruction.2.1.to_le_bytes().get(5).unwrap(), *instruction.2.1.to_le_bytes().get(6).unwrap(), *instruction.2.1.to_le_bytes().get(7).unwrap(),
                         *instruction.2.2.to_le_bytes().get(0).unwrap(), *instruction.2.2.to_le_bytes().get(1).unwrap(), *instruction.2.2.to_le_bytes().get(2).unwrap()];
    println!( "{}", res.map(|n| format!("{:02x?}", n) ).concat() );
}

fn write_raw( instruction: (u8, u8, (u64, u64, u64)) ){
    let res = [ instruction.0, instruction.1, 
                         *instruction.2.0.to_le_bytes().get(0).unwrap(), 
                         *instruction.2.1.to_le_bytes().get(0).unwrap(), *instruction.2.1.to_le_bytes().get(1).unwrap(), *instruction.2.1.to_le_bytes().get(2).unwrap(), *instruction.2.1.to_le_bytes().get(3).unwrap(),
                         *instruction.2.1.to_le_bytes().get(4).unwrap(), *instruction.2.1.to_le_bytes().get(5).unwrap(), *instruction.2.1.to_le_bytes().get(6).unwrap(), *instruction.2.1.to_le_bytes().get(7).unwrap(),
                         *instruction.2.2.to_le_bytes().get(0).unwrap(), *instruction.2.2.to_le_bytes().get(1).unwrap(), *instruction.2.2.to_le_bytes().get(2).unwrap()];
    println!( "{}", res.map(|n| format!("{}", n as char) ).concat() );
}