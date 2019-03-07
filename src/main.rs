extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::Parser;

#[derive(Parser)]
#[grammar = "zokrates.pest"]
pub struct ZoKratesParser;

fn main() {
    let input_string_2 = r#"def constant() -> (field):
  return 123123

def add(field a,field b) -> (field):
  a=constant()
  return a+b

def main(field a,field b) -> (field):
  field c = add(a, b+constant())
  return const()
"#;
    let parse2 =
        ZoKratesParser::parse(Rule::file, &input_string_2).map_err(|e| println!("{}", e)); // unwrap the parse result
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
