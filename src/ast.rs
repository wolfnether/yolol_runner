#[derive(Debug, PartialEq, Clone)]
pub enum Tree {
    Empty,
    Error,
    Comment(String),
    Numerical(i64),
    String(String),
    LocalVariable(String),
    GlobalVariable(String),
    Assign(Box<Tree>, Box<Tree>),
    AssignAdd(Box<Tree>, Box<Tree>),
    AssignSub(Box<Tree>, Box<Tree>),
    AssignMul(Box<Tree>, Box<Tree>),
    AssignDiv(Box<Tree>, Box<Tree>),
    AssignMod(Box<Tree>, Box<Tree>),
    AssignExp(Box<Tree>, Box<Tree>),
    IfThen(Box<Tree>, Vec<Tree>),
    Goto(Box<Tree>),
    Or(Box<Tree>, Box<Tree>),
    And(Box<Tree>, Box<Tree>),
    Eq(Box<Tree>, Box<Tree>),
    Ne(Box<Tree>, Box<Tree>),
    Lt(Box<Tree>, Box<Tree>),
    Gt(Box<Tree>, Box<Tree>),
    Lte(Box<Tree>, Box<Tree>),
    Gte(Box<Tree>, Box<Tree>),
    Add(Box<Tree>, Box<Tree>),
    Sub(Box<Tree>, Box<Tree>),
    Mul(Box<Tree>, Box<Tree>),
    Div(Box<Tree>, Box<Tree>),
    Mod(Box<Tree>, Box<Tree>),
    Exp(Box<Tree>, Box<Tree>),
    Abs(Box<Tree>),
    Sqrt(Box<Tree>),
    Sin(Box<Tree>),
    Cos(Box<Tree>),
    Tan(Box<Tree>),
    Asin(Box<Tree>),
    Acos(Box<Tree>),
    Atan(Box<Tree>),
    Not(Box<Tree>),
    PreDec(Box<Tree>),
    PreInc(Box<Tree>),
    PostDec(Box<Tree>),
    PostInc(Box<Tree>),
    Neg(Box<Tree>),
    Fac(Box<Tree>),
}
