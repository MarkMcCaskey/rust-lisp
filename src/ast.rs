use types::*;

#[derive(Debug)]
pub struct Program(Vec<Form>);

#[derive(Debug)]
pub enum Form {
    Def(Definition),
    Expr(Expression),
}

#[derive(Debug)]
pub enum Definition {
    Variable(VariableDefinition),
    //Syntax(SyntaxDefinition),
    BeginDefinitions(Vec<Definition>),
    LetSyntax(Vec<SyntaxBinding>, Vec<Definition>),
    LetRecSyntax(Vec<SyntaxBinding>, Vec<Definition>),
    //Derived(DerivedDefinition),
}

#[derive(Debug)]
pub enum VariableDefinition {
    Define(Variable, Expression),
    DefineMulti(Many1<Variable>, Body),
    DefineMultiDotted(Many1<Variable>, Variable, Body),
}

pub type Variable = Identifier;

#[derive(Debug)]
pub struct Body(Vec<Definition>, Many1<Expression>);

/*#[derive(Debug)]
pub struct SyntaxDefinition(Keyword, TransformerExpression);*/

pub type Keyword = Identifier;

//SyntaxBinding does not match formal Scheme grammar
#[derive(Debug)]
pub struct SyntaxBinding(Keyword, Expression);

#[derive(Debug)]
pub enum Expression {
    Const(Constant),
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

#[derive(Debug)]
pub enum Constant {
    Bool(bool),
    Num(Number),
    Char(char),
    Str(String),
}

#[derive(Debug)]
pub enum Formals {
    Var(Variable),
    ManyVar(Vec<Variable>),
    DottedManyVar(Many1<Variable>, Variable),
}

#[derive(Debug)]
pub struct Application(Expression, Vec<Expression>);

pub type Identifier = String;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum List {
    List(Vec<Datum>),
    Dotted(Many1<Datum>, Box<Datum>),
    Abbrev(Box<Abbreviation>),
}

#[derive(Debug)]
pub enum Abbreviation {
    Quote(Datum),
    BackTick(Datum),
    Eval(Datum),
    Splice(Datum),
}

#[derive(Debug)]
pub struct Vector(Vec<Datum>);

#[derive(Debug, PartialEq)]
pub struct Number(Complex);

#[derive(Debug, PartialEq)]
pub enum Complex {
    Real(Real),
    RealAtReal(Real, Real),
    RealPlusImag(Real, Imag),
    RealSubImag(Real, Imag),
    Imag(Imag),
}

#[derive(Debug, PartialEq)]
pub struct Imag(pub Real);

#[derive(Debug, PartialEq)]
pub struct Real(pub Num);

#[derive(Debug, PartialEq)]
pub enum Num {
    Float(f64),
    SInt(i64),
    UInt(u64),
    Ratio(i64, i64),
}

//TODO: fill these out
#[derive(Debug)]
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
