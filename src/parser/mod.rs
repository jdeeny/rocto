use nom::{alpha, space, IResult, hex_digit, digit};
use std::str::FromStr;

use chip8::instruction::{Src, Dest};

pub fn parse(input: &str) -> Vec<OctoFragment> {
    println!("\n\nInput {:?}\n\n", input);

    let parse = OctoParser::new();


    let w: IResult<&str, Vec<OctoFragment>> = parse.program(input).1; //program is a nom function

    println!("Total Lines: {}", parse.line_count() );

    if let IResult::Done(x, result) = w {
         println!("Done: x: {:?} result: {:?}", x, result);
         result
    } else {
        match w {
            IResult::Incomplete(n) => { println!("incomplete: {:?}", n); },
            IResult::Error(e) => { println!("error: {:?}", e); }
            _ => () ,
        };
        panic!("parse error");
    }
}

pub type LineNumber = usize;



#[derive(Debug, PartialEq)]
pub enum OctoSrc {
    Chip8(Src),
    Symbol(String)
}

#[derive(Debug, PartialEq)]
pub enum OctoDest {
    Chip8(Dest),
    Symbol(String)
}

#[derive(Debug, PartialEq)]
pub enum OctoAssignment {
    Store(OctoDest, OctoSrc),
    StoreRandom(OctoDest, usize),
    StoreHex(OctoDest, OctoSrc),
    Add(OctoDest, OctoSrc),
    Sub(OctoDest, OctoSrc),
    Or(OctoDest,  OctoSrc),
    And(OctoDest, OctoSrc),
    Xor(OctoDest, OctoSrc),
    Shr(OctoDest, OctoSrc),
    Shl(OctoDest, OctoSrc),
}

#[derive(Debug, PartialEq)]
pub enum OctoConditionalExpr {
    CondEq(OctoSrc, OctoSrc),
    CondNotEq(OctoSrc, OctoSrc),
    CondKey(OctoSrc),
    CondNotKey(OctoSrc),
}


#[derive(Debug, PartialEq)]
pub enum OctoStatement {
    Sprite(OctoSrc, OctoSrc, OctoSrc),
    Loop,
    Again,
    If(OctoConditionalExpr),
    Assignment(OctoAssignment),
}

#[derive(Debug, PartialEq)]
pub enum OctoFragment {
    Comment(LineNumber, String),
    //Codeblock(Vec<OctoFragment>),
    Alias(LineNumber, OctoDest, String),
    Const(LineNumber, isize, String),
    Label(LineNumber, String),
    Literal(LineNumber, isize),
    Statement(LineNumber, OctoStatement),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OctoParser {
    pub line_count: usize,
}

impl OctoParser {

    pub fn new() -> OctoParser {
        OctoParser { line_count: 0 }
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    method!(pub whitespace<OctoParser, &str, &str>, self,
        is_a_s!( " \t\r" )
    );

    method!(pub whitespace_or_newline<OctoParser, &str, &str>, mut self,
        alt_complete!(
            call_m!(self.whitespace)
            | call_m!(self.newline)
        )
    );

    method!(pub comment<OctoParser, &str, OctoFragment>, self,
        chain!(
            tag_s!("#")
            ~ s: is_not_s!("\n")
            , || OctoFragment::Comment(self.line_count, s.to_string())
        )
    );

    method!(pub newline<OctoParser, &str, &str>, mut self,
        map!( tag_s!("\n"), |s| { self.line_count += 1; s } )
    );


    method!(pub hex_digit_as_isize<OctoParser, &str, isize>, self,
        map_res!(hex_digit, FromStr::from_str)
    );

    method!(pub hex_val<OctoParser, &str, isize>, self,
        chain!(
            alt_complete!(
                tag_s!("0x")
                | tag_s!("0X")
            )
            ~ digits: hex_digit
            , || isize::from_str_radix(digits, 16).unwrap()
        )
    );


    method!(pub bin_val<OctoParser, &str, isize>, self,
        chain!(
            alt_complete!(
                tag_s!("0b")
                | tag_s!("0B")
            )
            ~ digits: is_a_s!("01")
            , || { println!("digits: {:?}", digits); isize::from_str_radix(digits, 2).unwrap() }
        )
    );

    method!(pub neg_val<OctoParser, &str, isize>, self,
        chain!(
            tag_s!("-")
            ~ val: map_res!(digit, isize::from_str)
            , || { -1 * val }
        )
    );

    method!(pub dec_val<OctoParser, &str, isize>, self,
        map_res!( digit, FromStr::from_str )
    );

    method!(pub val<OctoParser, &str, isize>, mut self,
        alt_complete!(
            call_m!(self.hex_val)
            | call_m!(self.bin_val)
            | call_m!(self.neg_val)
            | call_m!(self.dec_val)
        )
    );

    method!(pub dest_i<OctoParser, &str, OctoDest>, self,
        chain!(
            alt!( tag_s!("i") | tag_s!("I") )
            , || OctoDest::Chip8(Dest::I)
        )
    );


    method!(pub dest_symbol<OctoParser, &str, OctoDest>, mut self,
        map!( call_m!(self.symbol), |s: &str| { OctoDest::Symbol( s.to_string() ) } )
    );

    method!(pub src_symbol<OctoParser, &str, OctoSrc>, mut self,
        map!( call_m!(self.symbol), |s: &str| { OctoSrc::Symbol( s.to_string() ) } )
    );


    method!(pub dest_reg<OctoParser, &str, OctoDest>, mut self,
        chain!(
            alt!( tag_s!("v") | tag_s!("V") )
            ~ d: call_m!(self.hex_digit_as_isize)
            , || OctoDest::Chip8(Dest::Register(d as usize))
        )
    );

    method!(pub src_reg<OctoParser, &str, OctoSrc>, mut self,
        chain!(
            alt!( tag_s!("v") | tag_s!("V") )
            ~ d: call_m!(self.hex_digit_as_isize)
            , || OctoSrc::Chip8(Src::Register(d as usize))
        )
    );

    method!(pub src_val<OctoParser, &str, OctoSrc>, mut self,
        map!( call_m!(self.val), |v| OctoSrc::Chip8(Src::Const(v as usize)) )
    );

    method!(pub src_random<OctoParser, &str, OctoSrc>, mut self,
        chain!(
            alt!( tag_s!("r") | tag_s!("R") )
            ~ alt!( tag_s!("a") | tag_s!("A") )
            ~ alt!( tag_s!("n") | tag_s!("N") )
            ~ alt!( tag_s!("d") | tag_s!("D") )
            ~ alt!( tag_s!("o") | tag_s!("O") )
            ~ alt!( tag_s!("m") | tag_s!("M") )
            ~ many1!(call_m!(self.whitespace_or_newline))
            ~ call_m!(self.val)
            , || OctoSrc::Chip8(Src::Random)
        )
    );



    method!(pub label<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            tag_s!(":")
            ~ call_m!(self.whitespace)
            ~ name: call_m!(self.symbol)
            , || OctoFragment::Label(self.line_count, name.to_string())
        )
    );

    method!(pub symbol<OctoParser, &str, &str>, self,
        is_a_s!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")
    );

    method!(pub alias<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            tag_s!(":alias")
            ~ call_m!(self.whitespace)
            ~ ident: call_m!(self.symbol)
            ~ call_m!(self.whitespace)
            ~ r: call_m!(self.dest_reg)
            , || OctoFragment::Alias(self.line_count, r, ident.to_string())
        )
    );

    method!(pub lhs<OctoParser, &str, OctoDest>, mut self,
        alt_complete!(
            call_m!(self.dest_reg)
            | call_m!(self.dest_i)
            | call_m!(self.dest_symbol)
        )
    );


    method!(pub rhs<OctoParser, &str, OctoSrc>, mut self,
        alt_complete!(
            call_m!(self.src_reg)
            | call_m!(self.src_val)
            | call_m!(self.src_random)
            | call_m!(self.src_symbol)
        )
    );


    method!(pub assignment_add<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            dest: call_m!(self.lhs)
            ~ call_m!(self.whitespace)
            ~ tag_s!("+=")
            ~ call_m!(self.whitespace)
            ~ src: call_m!(self.rhs)
            , || OctoFragment::Statement(self.line_count, OctoStatement::Assignment(OctoAssignment::Add(dest, src)))
        )
    );


    method!(pub assignment_store<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            dest: call_m!(self.lhs)
            ~ call_m!(self.whitespace)
            ~ tag_s!(":=")
            ~ call_m!(self.whitespace)
            ~ src: call_m!(self.rhs)
            , || OctoFragment::Statement(self.line_count, OctoStatement::Assignment(OctoAssignment::Store(dest, src)))
        )
    );

    method!(pub assignment<OctoParser, &str, OctoFragment>, mut self,
        alt_complete!(
            call_m!(self.assignment_store)
            | call_m!(self.assignment_add)
        )
    );


    method!(pub sprite<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            alt!( tag_s!("s") | tag_s!("S") )
            ~ alt!( tag_s!("p") | tag_s!("P") )
            ~ alt!( tag_s!("r") | tag_s!("R") )
            ~ alt!( tag_s!("i") | tag_s!("I") )
            ~ alt!( tag_s!("t") | tag_s!("T") )
            ~ alt!( tag_s!("e") | tag_s!("E") )
            ~ call_m!( self.whitespace )
            ~ x: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ y: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ n: call_m!(self.src_val)
            , || OctoFragment::Statement(self.line_count, OctoStatement::Sprite(x, y, n))
        )
    );

    method!(pub loopstmt<OctoParser, &str, OctoFragment>, self,
        chain!(
            alt!( tag_s!("l") | tag_s!("L") )
            ~ alt!( tag_s!("o") | tag_s!("O") )
            ~ alt!( tag_s!("o") | tag_s!("O") )
            ~ alt!( tag_s!("p") | tag_s!("P") )
            , || OctoFragment::Statement(self.line_count, OctoStatement::Loop)
        )
    );

    method!(pub againstmt<OctoParser, &str, OctoFragment>, self,
        chain!(
            alt!( tag_s!("a") | tag_s!("A") )
            ~ alt!( tag_s!("g") | tag_s!("G") )
            ~ alt!( tag_s!("a") | tag_s!("A") )
            ~ alt!( tag_s!("i") | tag_s!("I") )
            ~ alt!( tag_s!("n") | tag_s!("N") )
            , || OctoFragment::Statement(self.line_count, OctoStatement::Again)
        )
    );


    method!(pub cond_eq<OctoParser, &str, OctoConditionalExpr>, mut self,
        chain!(
            lhs: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ tag_s!("==")
            ~ call_m!(self.whitespace)
            ~ rhs: alt!( call_m!(self.src_val) | call_m!(self.src_symbol) )
            , || OctoConditionalExpr::CondEq(lhs, rhs)
        )
    );

    method!(pub cond_noteq<OctoParser, &str, OctoConditionalExpr>, mut self,
        chain!(
            lhs: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ tag_s!("!=")
            ~ call_m!(self.whitespace)
            ~ rhs: alt!( call_m!(self.src_val) | call_m!(self.src_symbol) )
            , || OctoConditionalExpr::CondNotEq(lhs, rhs)
        )
    );

    method!(pub cond_key<OctoParser, &str, OctoConditionalExpr>, mut self,
        chain!(
            key: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ alt!( tag_s!("k") | tag_s!("K") )
            ~ alt!( tag_s!("e") | tag_s!("E") )
            ~ alt!( tag_s!("y") | tag_s!("Y") )
            , || OctoConditionalExpr::CondKey(key)
        )
    );

    method!(pub cond_notkey<OctoParser, &str, OctoConditionalExpr>, mut self,
        chain!(
            key: alt!( call_m!(self.src_reg) | call_m!(self.src_symbol) )
            ~ call_m!(self.whitespace)
            ~ tag_s!("-")
            ~ alt!( tag_s!("k") | tag_s!("K") )
            ~ alt!( tag_s!("e") | tag_s!("E") )
            ~ alt!( tag_s!("y") | tag_s!("Y") )
            , || OctoConditionalExpr::CondNotKey(key)
        )
    );


    method!(pub conditional_expr<OctoParser, &str, OctoConditionalExpr>, mut self,
        alt_complete!(
            call_m!(self.cond_eq)
            | call_m!(self.cond_noteq)
            | call_m!(self.cond_key)
            | call_m!(self.cond_notkey)
        )
    );

    method!(pub ifstmt<OctoParser, &str, OctoFragment>, mut self,
        chain!(
            alt!( tag_s!("i") | tag_s!("I") )
            ~ alt!( tag_s!("f") | tag_s!("F") )
            ~ call_m!(self.whitespace)
            ~ cond: call_m!(self.conditional_expr)
            ~ call_m!(self.whitespace)
            ~ alt!( tag_s!("t") | tag_s!("T") )
            ~ alt!( tag_s!("h") | tag_s!("H") )
            ~ alt!( tag_s!("e") | tag_s!("E") )
            ~ alt!( tag_s!("n") | tag_s!("N") )
            , || OctoFragment::Statement(self.line_count, OctoStatement::If(cond))
        )
    );


    method!(pub statement<OctoParser, &str, OctoFragment>, mut self,
        alt_complete!(
            call_m!(self.sprite)
            | call_m!(self.loopstmt)
            | call_m!(self.againstmt)
            | call_m!(self.ifstmt)
        )
    );


    method!(pub literal<OctoParser, &str, OctoFragment>, mut self,
        map!( call_m!(self.val), |v| OctoFragment::Literal(self.line_count, v) )
    );


    method!(pub fragment<OctoParser, &str, OctoFragment>, mut self,
        alt_complete!(
            //call_m!(self.whitespace)
            call_m!(self.comment)
            //| call_m!(self.newline)
            | call_m!(self.alias)
            | call_m!(self.label)
            | call_m!(self.assignment)
            | call_m!(self.statement)
            | call_m!(self.literal)
        )
    );



    method!(pub program<OctoParser, &str, Vec<OctoFragment> >, mut self,
        dbg!(
        many1!(
            chain!(
                many0!(call_m!(self.whitespace_or_newline))
                ~ frag: call_m!(self.fragment)
                ~ many0!(call_m!(self.whitespace_or_newline))
                , || frag
            )
        )
    )
    );

}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use chip8::instruction::{Src, Dest};

    use super::*;

    #[test]
    fn test_register() {
        assert_eq!( src_reg("v8"), IResult::Done("", OctoSrc::Chip8(Src::Register(8))) );
    }
    #[test]
    fn test_value() {
        assert_eq!( val("0x088"), IResult::Done("", 0x88) );
        assert_eq!( val("00"), IResult::Done("", 0) );
        assert_eq!( val("0"), IResult::Done("", 0) );
        assert_eq!( val("-45"), IResult::Done("", -45) );
        assert_eq!( val("12345"), IResult::Done("", 12345) );
        assert_eq!( val("0b0"), IResult::Done("", 0) );
        assert_eq!( val("0b1"), IResult::Done("", 1) );
        assert_eq!( val("0b0011"), IResult::Done("", 3) );
    }


}
