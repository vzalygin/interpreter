
pub type ParseResult<'a> = Option<(Compute<'a>, &'a str)>;

#[derive(Clone, Copy, Debug)]
pub enum Compute<'a> {
    Two(&'a str, &'a str),
    One(&'a str),
    Nothing,
}

pub fn gen_trivial<'a>(x: Compute<'a>) -> impl Fn(&'a str) -> ParseResult {
    move |text: &'a str| Some((x, text))
}

pub fn bind<'a>(
    parse: impl Fn(&'a str) -> ParseResult,
    gen: impl Fn(Compute<'a>) -> Box<dyn Fn(&'a str) -> ParseResult>,
) -> impl Fn(&'a str) -> Option<(Compute<'a>, &'a str)> {
    move |text: &'a str| match parse(text) {
        Some((value, tail)) => gen(value)(tail),
        None => None,
    }
}

pub fn alt<'a>(
    parsers: Vec<impl Fn(&'a str) -> ParseResult>,
) -> impl Fn(&'a str) -> ParseResult {
    move |text: &'a str| {
        for parser in parsers.iter() {
            let r = parser(text);
            if r.is_some() {
                return r;
            }
        }
        return None;
    }
}