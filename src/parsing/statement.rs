
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.expect("return")?;
        let expr = self.maybe( |input| input.parse_expr() );
        self.expect(";")?;
        Ok(Statement::Return(expr))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.choice( &[ |input| input.parse_return() ] )
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                      , |input| Ok(Expr::PString(input.parse_string()?))
                      , |input| input.parse_bool()

                      // TODO This needs to be last?
                      , |input| Ok(Expr::Variable(input.parse_symbol()?))
                      ] )
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }

    fn parse_bool(&mut self) -> Result<Expr, ParseError> {
        let rp = self.create_restore();
        let value = self.parse_symbol()?;
        if value.value == "true" {
            Ok(Expr::Bool(true))
        }
        else if value.value == "false" {
            Ok(Expr::Bool(false))
        }
        else {
            self.restore(rp);
            Err(ParseError::ErrorAt(value.start, "Expected boolean".to_string()))
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

}
