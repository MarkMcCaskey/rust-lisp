#![allow(dead_code)]

use types::*;
use ast;
use ast::*;
use std::str;
use std::str::FromStr;

use nom::*;

named!(pub form<Form>,
       alt!(do_parse!(d: definition >> (Form::Def(d))) |
            do_parse!(e: expression >> (Form::Expr(e)))));

named!(definition<Definition>,
       alt!(do_parse!(v: variable_definition >>
                      (Definition::Variable(v)))));

named!(variable_definition<VariableDefinition>,
       alt!(do_parse!(tag!("(") >>
                      opt!(multispace) >>
                      tag!("define") >>
                      opt!(multispace) >>
                      v: variable >>
                      opt!(multispace) >>
                      e: expression >>
                      opt!(multispace) >>
                      tag!(")") >>
                      (VariableDefinition::Define(v, e)))
       ));

named!(application<Application>,
       do_parse!(tag!("(") >>
                 opt!(multispace) >>
                 e: expression >>
                 opt!(multispace) >>
                 v: many0!(do_parse!(
                     se: expression >>
                         opt!(multispace) >>
                         (se))) >>
                 opt!(multispace) >>
                 tag!(")") >>
                 (Application(e, v))));

// TODO: quote (I don't like it being in the parser)
named!(pub expression<Expression>,
       alt_complete!(
           do_parse!(c: constant >> (Expression::Const(c))) | 
           do_parse!(v: variable >> (Expression::Variable(v))) |
           do_parse!(tag!("'") >> d: datum >>
                     (Expression::QuotedDatum(d))) |
           do_parse!(a: application >> (Expression::App(Box::new(a)))) 
       ));

named!(variable<Variable>,
       do_parse!(v: identifier >> (v)));

//TODO: finish
named!(constant<Constant>,
       alt!(do_parse!(b: boolean >> (Constant::Bool(b))) |
            do_parse!(n: number >> (Constant::Num(n))) |
            do_parse!(c: lisp_character >> (Constant::Char(c)))));
            

named!(identifier<Identifier>,
       do_parse!(i: initial >>
                 s: many0!(subsequent) >>
                 (s.iter().fold(str::from_utf8(i).unwrap().to_owned(),
                                |a, b| a + str::from_utf8(b).unwrap()))));

named!(initial<&[u8]>,
       alt!(alphanumeric |
            tag!(".") |
            tag!("+") |
            tag!("-") |
            tag!("!") |
            tag!("$") |
            tag!("%") |
            tag!("&") |
            tag!("*") |
            tag!("/") |
            tag!(":") |
            tag!("<") |
            tag!("=") |
            tag!(">") |
            tag!("?") |
            tag!("~") |
            tag!("_") |
            tag!("^") |
            tag!("@")));

named!(subsequent<&[u8]>,
       alt!(initial | tag!("#")));

named!(datum<Datum>, alt_complete!(
    do_parse!(b: boolean >> (Datum::Bool(b))) |
    do_parse!(tag!("#:") >>
              i: identifier >> (Datum::Sym(i))) |
    do_parse!(c: lisp_character >> (Datum::Char(c))) |
    do_parse!(c: complex >> (Datum::Num(c))) 
    ));

named!(lisp_character<char>,
       do_parse!(tag!("#\\") >>
                 v: alt!(anychar |
                         lisp_named_character) >>
                 (v)));

// TODO: finish this
named!(lisp_named_character<char>,
       alt!(value!('\n', tag!("newline")) | 
            value!(' ', tag!("space")) |
            value!('\0', tag!("nul")) |
            value!('\t', tag!("tab"))
       ));

named!(boolean<bool>, alt!(
    do_parse!(tag!("true") >> (true)) |
    do_parse!(tag!("false") >> (false))
    ));

named!(number<Number>, do_parse!(n: complex >> (Number(n))));

named!(complex<Complex>, alt!(
    do_parse!(r: real >>
              opt!(multispace) >>
              tag!("+") >>
              opt!(multispace) >>
              i: imag >>
              (Complex::RealPlusImag(r, i))) |
    do_parse!(r: real >>
              opt!(multispace) >>
              tag!("-") >>
              opt!(multispace) >>
              i: imag >>
              (Complex::RealSubImag(r, i))) |
    do_parse!(n: real >> (Complex::Real(n))) |
    do_parse!(n: imag >> (Complex::Imag(n)))
));
       
named!(imag<Imag>, alt!(
    do_parse!(
        tag!("i") >> (Imag(ast::Real(Num::Float(1.0))))) |
    do_parse!(
        n: real >>
            tag!("i") >>
            (Imag(n)))));
named!(real<ast::Real>, do_parse!(n: num >> (ast::Real(n))));
named!(num<Num>,
       alt_complete!(
           do_parse!(
               n1: parse_sint >>
                   opt!(multispace) >>
                   tag!("/") >>
                   opt!(multispace) >>
                   n2: parse_sint >>
                   (Num::Ratio(n1, n2))
           ) |
           do_parse!(n: parse_int >> (Num::UInt(n)))  |
           do_parse!(n: parse_sint >> (Num::SInt(n))) // |
           //do_parse!(n: be_f64 >> (Num::Float(n))) 
       ));

named!(parse_int<u64>, 
           map_res!(map_res!(digit, str::from_utf8), FromStr::from_str)
           );

//TODO: add sign support
named!(parse_sint<i64>, 
       alt!(
           do_parse!(
               tag!("-") >>
               num: parse_int >>
                   ((num as i64) * -1)) |
           map!(parse_int, |n| n as i64)));
#[test]
fn parse_num() {
    assert_eq!(num(b"12"), IResult::Done(&b""[..], Num::UInt(12)));
    assert_eq!(num(b"-12"), IResult::Done(&b""[..], Num::SInt(-12)));
}
#[test]
fn parse_frac() {
    assert_eq!(num(b"12 / 3"), IResult::Done(&b""[..], Num::Ratio(12, 3)));
}
#[test]
fn parse_complex() {
    assert_eq!(complex(b"123 + 435i"),
               IResult::Done(&b""[..],
                             Complex::RealPlusImag(
                                 ast::Real(Num::UInt(123)),
                                 ast::Imag(Real(Num::UInt(435))))));
}
