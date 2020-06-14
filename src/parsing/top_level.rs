
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
         
        // TODO maybe public?

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_fun_def(&mut self) -> Result<FunDef, ParseError> {
        
        self.expect("fun")?;
        
        let name = self.parse_symbol()?;

        self.expect("<")?; // TODO maybe

        let type_params = self.list(|input| input.parse_symbol())?;

        self.expect(">")?;


        self.expect("(")?;

        // parse parameters

        // parse return type maybe

        // parse { 

        // parse statements or exprs



        Err(ParseError::EndOfFile("".to_string()))
    }

}


#[cfg(test)]
mod test {
    use super::*;

}
