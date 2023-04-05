#![warn(missing_debug_implementations, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    divider: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(string: &'a str, divider: &'a str) -> Self {
        Self {
            remainder: string,
            divider,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_divider) = self.remainder.find(self.divider) {
            let until_divider = &self.remainder[..next_divider];
            self.remainder = &self.remainder[(next_divider + self.divider.len())..];
            Some(until_divider)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let string = "a f d h h j j";
    let letters = StrSplit::new(string, " ");
    assert!(letters.eq(vec!["a", "f", "d", "h", "h", "j", "j"].into_iter()));
}
