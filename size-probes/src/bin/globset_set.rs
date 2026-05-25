use globset::{Glob, GlobSetBuilder};

fn main() {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("**/*.rs").unwrap());
    let set = builder.build().unwrap();
    println!("{}", set.is_match("src/main.rs"));
}
