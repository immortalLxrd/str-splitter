// #![warn(missing_debug_implementations, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'string, D> {
    remainder: Option<&'string str>,
    divider: D,
}

impl<'string, D> StrSplit<'string, D> {
    pub fn new(string: &'string str, divider: D) -> Self {
        Self {
            remainder: Some(string),
            divider,
        }
    }
}

pub trait Divider {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'string, D> Iterator for StrSplit<'string, D>
where
    D: Divider,
{
    type Item = &'string str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.divider.find_next(remainder) {
            let until_divider = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_divider)
        } else {
            self.remainder.take()
        }
    }
}

impl Divider for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Divider for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    let divider = format!("{}", c);
    StrSplit::new(s, &*divider)
        .next()
        .expect("StrSplit always gives at least one result")
}

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
