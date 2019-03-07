extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "zokrates.pest"]
struct ZoKratesParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    ZoKratesParser::parse(Rule::file, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;

    mod examples {
        use super::*;
        extern crate glob;
        use glob::glob;
        use std::io::Read;

        #[test]
        fn examples_dir() {
            // Traverse all .code files in examples dir
            for entry in glob("examples/**/*.code").expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        println!("Parsing {:?}", path.display());
                        let mut file = fs::File::open(path).unwrap();
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap();
                        ZoKratesParser::parse(Rule::file, &data).unwrap();
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
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

        #[test]
        fn parse_invalid_identifier_because_keyword() {
            fails_with! {
                parser: ZoKratesParser,
                input: "endfor",
                rule: Rule::identifier,
                positives: vec![Rule::identifier],
                negatives: vec![],
                pos: 0
            };
        }

        #[test]
        fn parse_for_loop() {
            let input = "for field i in 0..3 do \n c = c + a[i] \n endfor";

            let parse = ZoKratesParser::parse(Rule::iteration_statement, input);
            assert!(parse.is_ok());
        }

    }
}
