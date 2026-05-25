fn main() {
    println!("{}", fast_glob::glob_match("**/*.rs", "src/main.rs"));
}
