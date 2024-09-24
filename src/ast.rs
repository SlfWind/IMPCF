
#[derive(Debug)]
pub enum CompUnit{
    Stmt(SeqStmt)
}

#[derive(Debug)]
pub enum SeqStmt {
    Seq(Box<SeqStmt>, Box<Stmt>), 
    Stmt(Box<Stmt>)
}

#[derive(Debug)]
pub enum Stmt{
    Skip(),
    Asgn(Variable, Exp),
    If(Exp, Box<SeqStmt>, Box<SeqStmt>),
    While(Exp, Box<SeqStmt>)
}

#[derive(Debug)]
pub enum Exp{
    Num(i32)
}

#[derive(Debug)]
pub enum Variable{
    Int(String)
}
