
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

    fn parse_tuple_type(&mut self) -> Result<Meta<Type>, ParseError> {
        
        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_fun_type(&mut self) -> Result<Meta<Type>, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_namespace_type(&mut self, initial : String) -> Result<(Vec<String>, String), ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_index_type(&mut self) -> Result<Type, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }
    
    pub fn parse_type(&mut self) -> Result<Meta<Type>, ParseError> {

        match self.parse_tuple_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        match self.parse_fun_type() {
            Ok(t) => return Ok(t),
            _ => (),
        }

        let Meta { value: simple, start: first_start, end: first_end } = self.parse_symbol()?;

        match self.expect("::") {
            Ok(_) => {
                let (namespace, symbol) = self.parse_namespace_type(simple)?;
                match self.expect("<") {
                    Ok(_) => {
                        let indices = self.parse_index_type()?;
                        Ok(Type::Namespace(namespace, Type::Index(symbol, indices)))
                    },
                    Err(_) => Ok(Type::Namespace(namespace, Type::Simple(symbol))),
                }
            },
            Err(_) =>
                match self.expect("<") {
                    Ok(_) => {
                        let indices = self.parse_index_type()?;
                        Ok(Type::Index(simple, indices))
                    },
                    Err(_) => Ok(Meta { start: first_start, end: first_end, value: Type::Simple(simple) }),
                },
        }
    }
}
