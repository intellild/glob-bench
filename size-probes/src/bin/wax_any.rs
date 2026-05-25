use wax::Program;

fn main() {
    let any = wax::any(["**/*.rs"]).unwrap();
    println!("{}", any.is_match("src/main.rs"));
}
