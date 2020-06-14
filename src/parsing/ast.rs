
#[derive(Debug)]
pub struct PSym {
    pub start : usize,
    pub end : usize,
    pub value : String,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Simple(PSym),
    Index(PSym, Vec<Type>),
    Fun { input : Vec<Type>, output : Box<Type> },
    Tuple(Vec<Type>),
    Namespace(Vec<PSym>, Box<Type>),
    Infer,
}

#[derive(Debug)]
pub struct Use {
    pub namespace : Vec<PSym>,
    pub imports : Vec<Import>,
}

#[derive(Debug)]
pub enum Import {
    Everything,
    Item(PSym),
}

#[derive(Debug)]
pub struct Mod {

}

#[derive(Debug)]
pub enum Expr {
    Number(PSym),
    PString(PSym),  
    Variable(PSym),
}

