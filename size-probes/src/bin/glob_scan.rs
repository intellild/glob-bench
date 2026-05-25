use glob::Pattern;

fn main() {
    let pattern = Pattern::new("**/*.rs").unwrap();
    println!("{}", pattern.matches("src/main.rs"));
}
