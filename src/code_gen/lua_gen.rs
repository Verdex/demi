
use crate::parsing::ast::*;
use super::code::Code;

pub fn gen_lua( code : Code ) -> String { 
    match code {
        Code::S(s) => s,
        _ => panic!("gen lua"),
    }
}

