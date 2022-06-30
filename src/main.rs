mod parser;

use parser::*;

fn parse_digit<'a>(text: &'a str) -> Option<(Compute<'a>, &'a str)> {
    if text.len() > 0 && text[0..1].parse::<u8>().is_ok() {
        Some((Compute::One(&text[0..1]), &text[1..]))
    } else {
        None
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "12 23 ";

        let parse_two_digit_number = bind(parse_digit, move |d1| {
            Box::new(bind(parse_digit, move |d2| {
                if let Compute::One(a) = d1 {
                    if let Compute::One(b) = d2 {
                        return Box::new(gen_trivial(Compute::Two(a, b)));
                    }
                }
                return Box::new(gen_trivial(Compute::Nothing));
            }))
        });

        let res = parse_two_digit_number(data);
        assert!(res.is_some());

        let (value, tail) = res.unwrap();
        assert_eq!(&data[2..], tail);
        assert!(match value {
            Compute::Two(a, b) => a == &data[0..1] && b == &data[1..2],
            _ => false,
        })
    }
}