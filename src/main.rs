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
    // let unparsed_file = fs::read_to_string("zoksample.code").expect("cannot read file");
    // let input_string = "for field i in 0..3 do \n c = c + a[i] \n endfor";

    // let parse = ZoKratesParser::parse(Rule::iteration_statement, &input_string)
    //     .expect("unsuccessful parse"); // unwrap the parse result
    // println!("{:#?}", parse);

    let input_string_2 = "for field i in 0..3 do \n c = c + a[i] \n endfor \n";
    let parse2 =
        ZoKratesParser::parse(Rule::statement, &input_string_2).expect("unsuccessful parse"); // unwrap the parse result
    println!("{:#?}", parse2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;

    mod examples {
        use super::*;
        extern crate glob;
        use glob::glob;

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
        fn parse_for_loop() {
            let input = "for field i in 0..3 do \n c = c + a[i] \n endfor";

            let parse = ZoKratesParser::parse(Rule::iteration_statement, input);
            assert!(parse.is_ok());

            // parses_to! {
            //     parser: ZoKratesParser,
            //     input : "for field i in 0..3 do \n c = c + a[i] \n endfor",
            //     rule: Rule::statement,
            //     tokens: [
            //         iteration_statement(0, 42,
            //             [
            //              type_name(4,9), identifier(10,11)
            //              ],
            //              [
            //                  constant(15,16),
            //              ],
            //             [
            //                 constant(18,19)
            //                 ],
            //         statement(26,41),
            //             )
            //         ]
            // };
        }

    }
}
