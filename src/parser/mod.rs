use nom::{alpha, space, IResult, hex_digit};
use std::str::FromStr;

pub fn parse(input: &str) -> Vec<OctoFragment> {
    println!("\n\nInput {:?}\n\n", input);

    let w = program(input); //program is a nom function

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

#[derive(Debug)]
pub enum OctoFragment {
    Whitespace(String),
    Newline,
    Comment(String),
    Codeblock(Vec<OctoFragment>),
    Alias(usize, String),
    Label(String),
    Assignment,
}


named!(pub whitespace(&str) -> OctoFragment,
    map!( is_a_s!( " \t\r" ), |s: &str| OctoFragment::Whitespace(s.to_string()) )
);

named!(pub comment(&str) -> OctoFragment,
    chain!(
        tag_s!("#")
        ~ s: is_not_s!("\n")
        , || OctoFragment::Comment(s.to_string())
    )
);

named!(pub newline(&str) -> OctoFragment,
    map!( tag_s!("\n"), |_| { OctoFragment::Newline } )
);


named!(pub hex_as_usize(&str) -> usize,
    map_res!(hex_digit, FromStr::from_str)
);

named!(pub register(&str) -> usize,
    chain!(
        alt!( tag_s!("v") | tag_s!("V") )
        ~ d: hex_as_usize
        , || d
    )
);


named!(pub identifier(&str) -> &str,
    is_a_s!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
);

named!(pub alias(&str) -> OctoFragment,
    chain!(
        tag_s!(":alias")
        ~ whitespace
        ~ ident: identifier
        ~ whitespace
        ~ r: register
        , || OctoFragment::Alias(r, ident.to_string())
    )
);


named!(pub fragment(&str) -> OctoFragment,
    alt_complete!( whitespace
        | comment
        | newline
        | alias
    )
);



named!(pub program(&str) -> Vec<OctoFragment>,
    dbg! (
        many1!(fragment)
/*        chain!
            ( whitespace?
            ~ tag_s!("#")
            ~ whitespace?
            ~ name: alpha
            , || name
        )*/
    )
);



named!(pub name_parser(&str) -> &str,
    chain!
        ( tag_s!("hello")
        ~ space?
        ~ name: alpha
        , || name
    )
);

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::*;

    #[test]
    fn test_register() {
//        assert_eq!( register("v8"), IResult::Done("", 8) );
    }
}
