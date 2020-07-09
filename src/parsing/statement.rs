
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
                      , |input| input.parse_lambda()

                      // TODO This needs to be last?
                      , |input| Ok(Expr::Variable(input.parse_symbol()?))
                      ] )
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
            let name = input.parse_symbol()?; 
            match input.expect(":") {
                Ok(_) => { 
                    let param_type = input.parse_type()?;
                    Ok(FunParam { name, param_type })
                },
                Err(_) => {
                    Ok(FunParam { name, param_type: Type::Infer })
                },
            }
        }
        self.expect("|")?;
        let params = self.list(parse_param)?;
        self.expect("|")?;
        match self.expect("->") {
            Ok(_) => {
                let return_type = self.parse_type()?;
                match self.expect("{") {
                    Ok(_) => {
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok(Expr::StatementLambda { params, return_type, definition })
                    },
                    Err(_) => {
                        let definition = Box::new(self.parse_expr()?);
                        Ok(Expr::ExprLambda { params, return_type, definition })
                    },
                }
            },
            Err(_) => {
                match self.expect("{") {
                    Ok(_) => {
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok(Expr::StatementLambda { params, return_type: Type::Infer, definition })
                    },
                    Err(_) => {
                        let definition = Box::new(self.parse_expr()?);
                        Ok(Expr::ExprLambda { params, return_type: Type::Infer, definition })
                    },
                }
            },
        }
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
