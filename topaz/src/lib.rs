pub mod tokenizer;
pub mod tokens;
pub mod ast;
pub mod context;
pub mod preprocessor;

pub fn tokenize<'input, T: AsRef<str>>( input: &'input T ) -> tokenizer::Tokenizer<'input> {
    tokenizer::Tokenizer::new( input.as_ref() ).into_iter()
}

#[cfg(test)]
#[test]
fn tokenize_constant_api(){
    use core::str;

    let input = String::from( "" );
    tokenize( &input );

    let _ = match  std::fs::File::open("example") {
        Ok( mut f ) => {
            use std::io::Read;
            let mut input = String::new();
            let _ = f.read_to_string( &mut input );
            tokenize( &input );
        },
        _ => {}
    };

    let static_input = "input";
    tokenize( &static_input );

    let raw_data  = str::from_utf8(&[0]).unwrap();
    tokenize( &raw_data );

}