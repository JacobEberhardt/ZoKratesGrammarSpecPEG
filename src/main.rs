extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::io::Read;

use pest::Parser;

#[derive(Parser)]
#[grammar = "zokrates.pest"]
pub struct ZoKratesParser;

fn main() {

    let unparsed_file = fs::read_to_string("zoksample.code").expect("cannot read file");

    let parse = ZoKratesParser::parse(Rule::file, &unparsed_file)
            .expect("unsuccessful parse"); // unwrap the parse result
    println!("{:?}", parse);
}


#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;

    mod examples {
        use super::*;
        #[test]
        fn examples_dir() {
            // TODO: Traverse examples dir recursively and read all .code files
            let mut dir = fs:: File::open(examples);
            let mut file = fs::File::open("examples/brackets.code").unwrap();
            let mut data = String::new();

            file.read_to_string(&mut data).unwrap();

            ZoKratesParser::parse(Rule::file, &data).unwrap();
        }
    }

    mod rules {
        use super::*;
        #[test]
        fn parse_valid_identifier() {
            parses_to! {
                parser: ZoKratesParser,
                input: "valididentifier_01",
                rule: Rule::identifier,
                tokens: [
                    identifier(0, 18)
                ]
            };
        }

        #[test]
        fn parse_invalid_identifier() {
            fails_with! {
                parser: ZoKratesParser,
                input: "0_invalididentifier",
                rule: Rule::identifier,
                positives: vec![Rule::identifier],
                negatives: vec![],
                pos: 0
            };
        }

    }
}