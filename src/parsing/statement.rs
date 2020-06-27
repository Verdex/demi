
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;
use super::misc;

impl<'a> Input<'a> {

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.expect("return")?;
        let expr = self.maybe( |input| input.parse_expr() );
        Ok(Statement::Return(expr))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.choice( &[ |input| input.parse_return() ] )
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                      , |input| Ok(Expr::PString(input.parse_string()?))

                      // TODO This needs to be last?
                      , |input| Ok(Expr::Variable(input.parse_symbol()?))
                      ] )
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {

        Err(ParseError::EndOfFile("".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
