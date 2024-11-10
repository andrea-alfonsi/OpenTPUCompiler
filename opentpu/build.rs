use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("assembler.y")
                .unwrap()
        })
        .lexer_in_src_dir("assembler.l")
        .unwrap()
        .build()
        .unwrap();
}