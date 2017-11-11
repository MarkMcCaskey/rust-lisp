use types::*;
use ast;
use ast::*;

use nom::*;

named!(complex<Complex>, alt!(
    do_parse!(n1: real >>
              opt!(multispace) >>
              tag!("@") >>
              opt!(multispace) >>
              n2: real >>
              (Complex::RealAtReal(n1, n2))) |

    do_parse!(n: real >> (Complex::Real(n))) 
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
       alt!(
           do_parse!(
               n1: be_i64 >>
                   opt!(multispace) >>
                   tag!("/") >>
                   opt!(multispace) >>
                   n2: be_i64 >>
                   (Num::Ratio(n1, n2))
           ) |
           do_parse!(n: be_u64 >> (Num::UInt(n)))  |
           do_parse!(n: be_i64 >> (Num::SInt(n)))  |
           do_parse!(n: be_f64 >> (Num::Float(n))) 
       ));

#[test]
fn parse_num() {
    assert_eq!(num(b"12"), IResult::Done(&b""[..], Num::UInt(12)));
    assert_eq!(num(b"12 / 3"), IResult::Done(&b""[..], Num::Ratio(12, 3)));
    assert_eq!(complex(b"123 @ 43555555"),
               IResult::Done(&b""[..],
                             Complex::RealAtReal(
                                 ast::Real(Num::UInt(123)),
                                 ast::Real(Num::UInt(43555555)))));
}
