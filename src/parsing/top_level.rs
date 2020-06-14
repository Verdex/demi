
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
         
        // TODO maybe public?

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_fun_def(&mut self) -> Result<FunDef, ParseError> {
        fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
            let name = input.parse_symbol()?; 
            input.expect(":")?;
            let param_type = input.parse_type()?;
            Ok(FunParam { name, param_type })
        }
        
        self.expect("fun")?;
        
        let name = self.parse_symbol()?;

        match self.maybe(|input| input.expect("<"))? {
            Some(_) => {
                let type_params = self.list(|input| input.parse_symbol())?;
                self.expect(">")?;
                self.expect("(")?;
                let params = self.list(parse_param)?;
                self.expect(")")?;
                // TODO return type maybe?
                // parse { 

                // parse statements or exprs

            },
            None => {

            },
        }

        Err(ParseError::EndOfFile("".to_string()))
    }

}


#[cfg(test)]
mod test {
    use super::*;

}
