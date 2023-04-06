use strsplitter::StrSplit;

#[test]
fn it_works() {
    let string = "a f d h h j j";
    let letters: Vec<_> = StrSplit::new(string, " ").collect();
    assert_eq!(letters, vec!["a", "f", "d", "h", "h", "j", "j"]);
}

#[test]
fn tail() {
    let string = "a f d h h j ";
    let letters: Vec<_> = StrSplit::new(string, " ").collect();
    assert_eq!(letters, vec!["a", "f", "d", "h", "h", "j", ""]);
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Until char", 't'), "Un");
}

fn until_char(s: &str, c: char) -> &str {
    let divider = format!("{}", c);
    StrSplit::new(s, &*divider)
        .next()
        .expect("StrSplit always gives at least one result")
}
