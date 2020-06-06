
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    /* 

    (a,b,c)
    a
    a::b::c
    fun(a,b,c) -> d
    a<b>
    ()

    */

    fn parse_tuple_type(&mut self) -> Result<Type, ParseError> {
        
        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_fun_type(&mut self) -> Result<Type, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_namespace_type(&mut self, initial : PSym) -> Result<(Vec<PSym>, PSym), ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_index_type(&mut self, init : PSym) -> Result<Type, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }
    
    pub fn parse_type(&mut self) -> Result<Type, ParseError> {

        match self.parse_tuple_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        match self.parse_fun_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        let simple = self.parse_symbol()?;

        match self.expect("::") {
            Ok(_) => {
                let (namespace, symbol) = self.parse_namespace_type(simple)?;
                match self.expect("<") {
                    Ok(_) => {
                        let index_type = self.parse_index_type(symbol)?;
                        Ok(Type::Namespace(namespace, Box::new(index_type)))
                    },
                    Err(_) => Ok(Type::Namespace(namespace, Box::new(Type::Simple(symbol)))),
                }
            },
            Err(_) =>
                match self.expect("<") {
                    Ok(_) => Ok(self.parse_index_type(simple)?),
                    Err(_) => Ok(Type::Simple(simple)),
                },
        }
    }
}
