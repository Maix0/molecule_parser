use molecule_parser;
fn main() {
    println!(
        "{:#?}",
        molecule_parser::Compound::parse(&std::env::args().nth(1).unwrap())
    );
}
