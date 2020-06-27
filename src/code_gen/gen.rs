
use crate::parsing::ast::*;
use super::code_gen_error::CodeGenError;
use super::code::Code;

pub fn gen_mod( module : Mod ) -> Result<Code, CodeGenError> { 
    "".to_string()
}

fn gen_fun_def( fun_def : FunDef ) -> Result<Code, CodeGenError> {

}
