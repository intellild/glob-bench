use globset::Glob;
use pcre2::bytes::RegexBuilder;

fn main() {
    let regex = Glob::new("**/*.rs").unwrap().regex().replace("(?-u)", "");
    let mut builder = RegexBuilder::new();
    builder.jit_if_available(true);
    let regex = builder.build(&regex).unwrap();
    println!("{}", regex.is_match("src/main.rs".as_bytes()).unwrap());
}
