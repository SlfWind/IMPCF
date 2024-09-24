use lalrpop_util::lalrpop_mod;
use std::{env, fs};

pub mod ast;
lalrpop_mod!(pub imp);

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = match args.get(1){
        Some(p) => p.clone(),
        None => panic!("No input file!")
    };
    let input_file = fs::read_to_string(input_path).unwrap();
    let ast = imp::CompUnitParser::new().parse(&input_file).unwrap();
    println!("{:#?}", ast);

}
