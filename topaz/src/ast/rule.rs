use std::collections::HashMap;

use crate::tokens::Token;

use crate::ast::variable::Variable;
use crate::ast::variable_type::VariableType;

#[derive( Debug, PartialEq, Copy, Clone)]
pub struct RuleNumber<'src> { 
    pub value: &'src str
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RuleLeftItem<'src> {
    Word( &'src str ),
    Variable( Variable<'src> )
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RuleRightItem<'src> {
    Variable( Variable<'src> ),
    Number( RuleNumber<'src>  )
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule<'src> {
    pub left_items: Vec<RuleLeftItem<'src>>,
    pub right_items : Vec<RuleRightItem<'src>>
}

impl<'src> Rule<'src> { 
    pub fn get_hint( &self ) -> String {
        self.left_items.iter().map(|item| match item {
            RuleLeftItem::Word( word) => format!("{}", word),
            RuleLeftItem::Variable(variable) => format!("{{{}}}", variable.name),
        }).collect::<Vec<String>>().join(" " )
    }

    pub fn build_from_tokens<T: Iterator<Item = Token<'src>>>(
        s: &mut std::iter::Peekable<T>,
    ) -> Result<Self, &'static str> {
        let mut rule = Self { left_items: vec![], right_items: vec![] };
        let mut variables_map = HashMap::<&'src str, Variable<'src>>::new();
        let mut is_left = true;

        while let Some(token) = s.peek() {
            match token {
                Token::Error => return Err("Cannot build Rule because got an error token"),
                Token::Whitespace( _ ) => {  s.next(); }
                Token::Comment(_) => { s.next(); }
                Token::Word( w ) => {
                    let word = *w;
                    s.next();
                    if is_left {
                        rule.left_items.push( RuleLeftItem::Word( word ));
                    } else{
                        return Err("Unexpected word in the right side of the rule")
                    }
                }
                Token::BraceOpen => {
                    s.next();
                    if is_left {
                        let variable = Variable::build_from_tokens( s ).unwrap();
                        while let Some( token ) = s.peek() {
                            match token {
                                Token::BraceClose => { s.next(); break; }
                                Token::Whitespace( _ ) => { s.next(); }
                                _ => return Err("unexpeted token")
                            }
                        }
                        if s.peek() == None {
                            return Err("Expected closing brace");
                        }
                        rule.left_items.push( RuleLeftItem::Variable( variable ) );
                        variables_map.insert( &variable.name, variable );
                    } else {
                        let mut variable_already_found = false;
                        while let Some( token ) = s.peek() {
                            match token {
                                Token::Whitespace( _ ) => { s.next(); },
                                Token::Word(  n ) => {
                                    let name = *n;
                                    s.next();
                                    if variable_already_found {
                                        return Err("Only one variable between braces is expected")
                                    }
                                    variable_already_found = true;
                                    let variable = variables_map.get( name ).unwrap();
                                    rule.right_items.push( RuleRightItem::Variable( variable.clone() ));
                                },
                                Token::BraceClose => {
                                    s.next();
                                    break;
                                },
                                _ => return Err("Unexpected token")
                                
                            }
                        } if s.peek() == None {
                            return Err("Unexpected EOF")
                        }
                    }
                },
                Token::HeavyArrowRight => { s.next(); is_left = false; },
                Token::LineBreak => return Ok( rule ),
                Token::Number( n ) => {
                    let value = *n;
                    s.next();
                    if is_left {
                        return Err("Unexpected number ofthe left side of the rule")
                    } else {
                        rule.right_items.push( RuleRightItem::Number( RuleNumber { value } )  );
                    }
                }
                _ => return Err("Unexpected token")
                
            }
        }
        Err("Unexpected EOF while matching Rule directive")
    }

    /// Try matching the code wih the rule or return the difference score
    pub fn try_assemble( &self, instruction: &crate::ast::instruction::AsmInstruction ) -> Result<Vec<u8>, u32> {
        fn calculate_words_difference( w1: &str, w2: &str) -> u32 {
            let mut counter: HashMap<char, i32> = HashMap::new();
            for c in w1.chars() {
                *counter.entry(c).or_insert(0) += 1; // one more
            }
            for c in w2.chars() {
                *counter.entry(c).or_insert(0) -= 1; // one less
            }
            counter
                .iter()
                .filter(|(_c, &n)| n > 0)
                .fold(0, |acc, v| acc + v.1.unsigned_abs())
        }

        let mut result = Vec::<u8>::new();
        let mut variables = HashMap::<&'src str, &'src str>::new();
        let mut total_score = 0;
        for (i, rule_item) in self.left_items.iter().enumerate() {
            let score = match rule_item {
                RuleLeftItem::Word( word) => {
                    match instruction.instructions[i] {
                        Token::Word( iword ) => { if *word == iword { Ok(10) } else { Err(calculate_words_difference(word, iword)) }}
                        _ => { Err(0) }
                    }
                },
                RuleLeftItem::Variable(variable) => { 
                    match instruction.instructions[i] {
                        Token::Number( value ) => { 
                            variables.insert( &variable.name, value );
                            Ok( 10 )
                        }
                        _ => { Err(0) }
                    }
                }
            };
            match score {
                Ok( s ) => total_score += s,
                Err( s ) => return Err(total_score + s)
            }
        }
        for right in &self.right_items {
            match right {
                RuleRightItem::Variable(variable) => {
                    // Variable mst exists othrwise rule cannot be built
                    let value = variables.get( variable.name ).unwrap();
                    if value.starts_with("0x") {
                        result.extend(
                            (0..value[2..].len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&value[i + 2..i + 4], 16).unwrap() )
                            .collect::<Vec<u8>>()
                        );
                    } else {
                        let bytes = match variable.v_type {
                            VariableType::U8 => u8::from_str_radix( &value, 10).unwrap().to_be_bytes().to_vec(),
                            VariableType::U16 => u16::from_str_radix( &value, 10).unwrap().to_be_bytes().to_vec(),
                            VariableType::U32 => u32::from_str_radix( &value, 10).unwrap().to_be_bytes().to_vec(),
                            VariableType::U64 => u64::from_str_radix( &value, 10).unwrap().to_be_bytes().to_vec()
                        };
                        result.extend(
                            bytes
                        );
                    }
                    
                },
                RuleRightItem::Number(rule_number) => {
                    if  rule_number.value.starts_with("0x") {
                        result.extend(
                            (0..rule_number.value[2..].len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&rule_number.value[i + 2..i + 4], 16).unwrap() )
                            .collect::<Vec<u8>>()
                          );
                    } else {
                        todo!("Non hex number are not supported yet")
                    }
                },
            }
        }
        Ok( result )
    }

    pub fn try_disassemble( &self, raw: &Vec<u8> )  -> Option<String> {
        let mut result = String::new();
        let mut variables = HashMap::<&'src str, (VariableType, Option<Vec<u8>>)>::new();
        let mut byte_iter = raw.iter();

        for item in self.left_items.iter() {
            match item {
                RuleLeftItem::Variable(variable) => {
                    variables.insert(variable.name, (variable.v_type, None));
                },
                _ => {}
            }
        }

        for item in self.right_items.iter(){
            match item {
                RuleRightItem::Number(rule_number) => {
                    if rule_number.value.starts_with("0x") {
                        let hex =  (0..rule_number.value[2..].len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&rule_number.value[i + 2..i + 4], 16).unwrap() )
                            .collect::<Vec<u8>>();
                        for x in hex {
                            match byte_iter.next() {
                                Some( h ) => {
                                    if *h != x {
                                        return None
                                    }
                                },
                                None => return None,
                            }
                        }
                    } else {
                        todo!("Non hex number are not supported yet")
                    }
                },
                RuleRightItem::Variable(variable) => {
                    match variables.get_mut( variable.name ) {
                        None => { return None },
                        Some( val) => {

                            match val.0 {
                                VariableType::U8 => {
                                    let value = match byte_iter.next() {
                                        None => return None,
                                        Some( b ) => b 
                                    };
                                    if let Some( v ) = &val.1 {
                                        if v.len() == 1 && v[0] != *value {
                                            return None
                                        }
                                    } else {
                                        val.1 = Some( vec![*value] );
                                    }
                                },
                                VariableType::U16 => {
                                    let value0 = match byte_iter.next() {
                                        None => return None,
                                        Some( b ) => b 
                                    };
                                    let value1 = match byte_iter.next() {
                                        None => return None,
                                        Some( b ) => b 
                                    };
                                    if let Some( v ) = &val.1 {
                                        if v.len() == 2 && (v[0] != *value0 || v[1] != *value1) {
                                            return None
                                        }
                                    } else {
                                        val.1 = Some( vec![*value0, *value1] );
                                    }
                                },
                                VariableType::U32 => {
                                    let mut values = Vec::with_capacity(4);
                                    for i in 0..4 {
                                        values[i] =  match byte_iter.next() {
                                            None => return None,
                                            Some( b ) => *b 
                                        };
                                    }
                                    if let Some( v ) = &val.1 {
                                        if v != &values {
                                            return None
                                        }
                                    } else {
                                        val.1 = Some( values );
                                    }
                                },
                                VariableType::U64 => {
                                    let mut values = Vec::with_capacity(8);
                                    for i in 0..8 {
                                        values[i] =  match byte_iter.next() {
                                            None => return None,
                                            Some( b ) => *b 
                                        };
                                    }
                                    if let Some( v ) = &val.1 {
                                        if v != &values {
                                            return None
                                        }
                                    } else {
                                        val.1 = Some( values );
                                    }
                                },
                            };
                        },
                    }
                }
            }
        }

        for item in &self.left_items {
            match item {
                RuleLeftItem::Word( w ) => result.push_str( w ),
                RuleLeftItem::Variable(variable) => {
                    if let Some ( var ) = variables.get( variable.name ) {
                        if let Some( v ) = &var.1  {
                            result.push_str( "0x") ;
                            result.push_str( &v.iter().map(|f|{ format!("{:02x?}" , f) }).collect::<String>() );
                        }
                    }
                }
            }
            result.push( ' ' );
        }
        Some( result )
    }
}