// fn gen_parse_char<'a>(x: &'a str) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
//     |text: &'a str| {
//         if text.len() > 0 && text[0..1] == *x {
//             Some((&text[0..1], &text[1..]))
//         } else {
//             None
//         }
//     }
// }

enum Something<'a> {
    TwoDigits(&'a str, &'a str)
}

fn gen_trivial<'a>(x: &'a str) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    |text: &'a str| Some((x, text))
}

fn parse_digit<'a>(text: &'a str) -> Option<(&'a str, &'a str)> {
    if text.len() > 0 && text[0..1].parse::<u8>().is_ok() {
        Some((&text[0..1], &text[1..]))
    } else {
        None
    }
}

fn bind<'a>(
    parse: impl Fn(&'a str) -> Option<(&'a str, &'a str)>,
    gen: impl Fn(&'a str) -> Box<dyn Fn(&'a str) -> Option<(&'a str, &'a str)>>,
) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |text: &'a str| match parse(text) {
        Some((value, tail)) => gen(value)(tail),
        None => None,
    }
}

fn main() {
    let parse_two_digit_number = 
            bind(parse_digit, |d1|
        Box::new(bind(parse_digit, |d2| 
        Box::new(gen_trivial(d2)))
        ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "12 ";

        let parse_two_digit_number = 
                 bind(parse_digit, |d1|
        Box::new(bind(parse_digit, |d2| 
        Box::new(gen_trivial(d2)))
        ));

        let res = parse_two_digit_number(data);
        assert_eq!(true, res.is_some());
        let (value, tail) = res.unwrap();
        assert_eq!(data[0..1], *value);
        assert_eq!(data[2..], *tail);
    }
}
