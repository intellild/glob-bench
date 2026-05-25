use globset::Glob;
use regex::bytes::RegexSet;

fn main() {
    let regex = Glob::new("**/*.rs").unwrap().regex().to_owned();
    let set = RegexSet::new([regex]).unwrap();
    println!("{}", set.is_match("src/main.rs".as_bytes()));
}
