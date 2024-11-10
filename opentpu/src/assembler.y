%start Instruction
%%


Instruction -> Result<(u8, u8, (u64, u64, u64)), ()>:
      MnemonicWithArgs { $1 }
    | PlainMnemonic { Ok(( $1?, 0, (0, 0, 0) )) }
    | Instruction 'COMMENT' { $1 }
    ;

MnemonicWithArgs -> Result<( u8, u8, (u64, u64, u64) ), ()>:
      'WRITE'                    Number Number Number { Ok((2 , 0, ($2?, $3?, $4?))) }
    | 'SET'                      Number Number        { Ok((3 , 0, ($2?, $3?, 0  ))) }
    | 'READ'                     Number Number Number { Ok((4 , 0, ($2?, $3?, $4?))) }
    | 'LOAD'                     Number Number Number { Ok((5 , 0, ($2?, $3?, $4?))) }
    | 'GET'                      Number Number        { Ok((6 , 0, ($2?, $3?, 0  ))) }
    | 'MAC'                      Number Number Number { Ok((8 , 0, ($2?, $3?, $4?))) }
    | 'ACTIVATE' 'DOT' 'RELU'    Number Number Number { Ok((12, 1, ($4?, $5?, $6?))) }
    | 'ACTIVATE' 'DOT' 'SOFTMAX' Number Number Number { Ok((12, 2, ($4?, $5?, $6?))) }
    ;

PlainMnemonic -> Result<u8, ()>:
      'NOOP' { Ok( 0 ) }
    | 'HALT' { Ok( 1 ) }
    | 'SYNC' { Ok( 7 ) }
    ;

Number -> Result<u64, ()>:
      Hex { $1 }
    | Int { $1 }
    ;

Hex -> Result<u64, ()>:
    'HEX'
      {
          let v = $1.map_err(|_| ())?;
          parse_hex($lexer.span_str(v.span()))
      }
    ;

Int -> Result<u64, ()>:
    'INT'
      {
          let v = $1.map_err(|_| ())?;
          parse_int($lexer.span_str(v.span()))
      }
    ;

%%
// Any functions here are in scope for all the grammar actions above.

fn parse_hex(s: &str) -> Result<u64, ()> {
    match u64::from_str_radix( s.trim_start_matches("0x"), 16 ){
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}

fn parse_int(s: &str) -> Result<u64, ()> {
    match s.parse::<u64>(){
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}