use types::*;

#[derive(Debug)]
pub struct Program(Vec<Form>);

#[derive(Debug, Clone)]
pub enum Form {
    Def(Definition),
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum Definition {
    Variable(VariableDefinition),
    //Syntax(SyntaxDefinition),
    BeginDefinitions(Vec<Definition>),
    LetSyntax(Vec<SyntaxBinding>, Vec<Definition>),
    LetRecSyntax(Vec<SyntaxBinding>, Vec<Definition>),
    //Derived(DerivedDefinition),
}

#[derive(Debug, Clone)]
pub enum VariableDefinition {
    Define(Variable, Expression),
    DefineMulti(Many1<Variable>, Body),
    DefineMultiDotted(Many1<Variable>, Variable, Body),
}

pub type Variable = Identifier;

#[derive(Debug, Clone)]
pub struct Body(Vec<Definition>, Many1<Expression>);

/*#[derive(Debug)]
pub struct SyntaxDefinition(Keyword, TransformerExpression);*/

pub type Keyword = Identifier;

//SyntaxBinding does not match formal Scheme grammar
#[derive(Debug, Clone)]
pub struct SyntaxBinding(Keyword, Expression);

#[derive(Debug, Clone)]
pub enum Expression {
    Const(Constant),
    Variable(Variable),
    QuotedDatum(Datum),
    Lambda(Formals, Body),
    IfElse {
        boolean_expr: Box<Expression>,
        true_branch: Box<Expression>,
        false_branch: Box<Expression>,
    },
    If {
        boolean_expr: Box<Expression>,
        true_branch: Box<Expression>,
    },
    Set(Variable, Box<Expression>),
    App(Box<Application>),
    Let(Vec<SyntaxBinding>, Many1<Expression>),
    LetRec(Vec<SyntaxBinding>, Many1<Expression>),
    Derived(DerivedExpression),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Bool(bool),
    Num(Number),
    Char(char),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Formals {
    Var(Variable),
    ManyVar(Vec<Variable>),
    DottedManyVar(Many1<Variable>, Variable),
}

#[derive(Debug, Clone)]
pub struct Application(pub Expression, pub Vec<Expression>);

pub type Identifier = String;

#[derive(Debug, Clone)]
pub enum Datum {
    Bool(bool),
    Num(Complex),
    Char(char),
    Str(String),
    Sym(Symbol),
    LitList(List),
    Vec(Vector),
}

pub type Symbol = Identifier;

#[derive(Debug, Clone)]
pub enum List {
    List(Vec<Datum>),
    Dotted(Many1<Datum>, Box<Datum>),
    Abbrev(Box<Abbreviation>),
}

#[derive(Debug, Clone)]
pub enum Abbreviation {
    Quote(Datum),
    BackTick(Datum),
    Eval(Datum),
    Splice(Datum),
}

#[derive(Debug, Clone)]
pub struct Vector(pub Vec<Datum>);

#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub Complex);

#[derive(Debug, PartialEq, Clone)]
pub enum Complex {
    Real(Real),
    RealPlusImag(Real, Imag),
    RealSubImag(Real, Imag),
    Imag(Imag),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Imag(pub Real);

#[derive(Debug, PartialEq, Clone)]
pub struct Real(pub Num);

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Float(f64),
    SInt(i64),
    UInt(u64),
    Ratio(i64, i64),
}

//TODO: fill these out
#[derive(Debug, Clone)]
pub enum DerivedExpression {
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Begin,
    Case,
    Cond,
    Delay,
    Do(Vec<Expression>),
    Let,
    LetStar,
    LetRec,
    QuasiQuote,
}
