
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // TODO let
        // TODO match
        // TODO set
        // TODO foreach
        // TODO while
        self.choice( &[ |input| input.parse_return() ] )
    }

    fn parse_call_statement(&mut self) -> Result<Statement, ParseError> {
    // a.blah()-ikky() => ikky(a["blah"]())
    // a-ikky().blah() => ikky(a)["blah"]()
        Err(ParseError::ErrorAt(0, "".to_string()))
    }

    fn parse_hyphen_call_statement(&mut self) -> Result<Statement, ParseError> {
        Err(ParseError::ErrorAt(0, "".to_string()))
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.expect("return")?;
        let expr = self.maybe( |input| input.parse_expr() );
        self.expect(";")?;
        Ok(Statement::Return(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {

                      // TODO call
                      // TODO dot call (a-blah() => blah(a))
                      // TODO dot (table index)
                      
        let expr = self.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                      , |input| Ok(Expr::PString(input.parse_string()?))
                      , |input| input.parse_bool()
                      , |input| input.parse_lambda()
                      
                      
                      // TODO namespace symbol 

                      // TODO This needs to be last?
                      , |input| Ok(Expr::Variable(input.parse_symbol()?))
                      ] )?;

        self.parse_post_expr(expr) // TODO one or more
    }

    fn parse_call_expr(&mut self) -> Result<Expr, ParseError> {
        let rp = self.create_restore();
        let func = Box::new(self.parse_expr()?);

        match self.expect("(") {
            _ => {},
            Err(e) => { self.restore(rp); return Err(e); },
        }

        let params = self.list(|input| input.parse_expr())?;

        self.expect(")")?;

        Ok(Expr::Call{ func, params })
    }

    // hypen call, call, dot
    fn parse_post_expr(&mut self, e : Expr) -> Result<Expr, ParseError> {
        match self.expect("-") {
            Ok(_) => panic!(""),
            Err(_) => {
                match self.expect("(") {
                    Ok(_) => {
                        let params = self.list(|input| input.parse_expr())?;

                        self.expect(")")?; 
                    
                        Ok(Expr::Call { func: Box::new(e), params })
                    },
                    Err(_) => {
                        match self.expect(".") {
                            Ok(_) => panic!(""),
                            Err(_) => return Ok(e),
                        }
                    },
                }
            },
        }
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

    #[test]
    fn should_parse_expr_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a : A, b : B, c| -> R { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_expr_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a, b : B, c : C| -> R<T> 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call() -> Result<(), ParseError> {
        let i = r#"x()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
        assert!( matches!( u, Expr::Call { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call_call() -> Result<(), ParseError> {
        let i = r#"x()()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
        let call = match u {
           Expr::Call { func, .. } => *func, 
           e => panic!("expected call but found {:?}", e),
        };

        assert!( matches!( call, Expr::Call { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call_call_with_param() -> Result<(), ParseError> {
        let i = r#"x(a)(b)"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
        let call = match u {
           Expr::Call { func, .. } => *func, 
           e => panic!("expected call but found {:?}", e),
        };

        // TODO check a and b

        assert!( matches!( call, Expr::Call { .. } ) );
        Ok(())
    }
}
