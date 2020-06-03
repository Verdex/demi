
#[derive(Debug)]
pub struct Meta<T> {
    pub start : usize,
    pub end : usize,
    pub value : T,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Simple(String),
    Index(String, Vec<Type>),
    Fun { input : Vec<Type>, output : Box<Type> },
    Tuple(Vec<Type>),
    Namespace(Vec<String>, Box<Type>),
    Infer,
}

#[derive(Debug)]
pub struct Mod {

}

#[derive(Debug)]
pub enum Expr {

}
