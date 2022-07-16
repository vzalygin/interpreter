#![feature(let_chains)]

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Node,
    Number(i32),
    Word(String),
    None,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    child: Token,
    tail: String,
}

impl Node {
    fn new(child: Token, tail: String) -> Node {
        Node { 
            child,
            tail
        }
    }
}

type ParsingResult = Result<Node, ()>;
type Parser = Box<dyn Fn(String) -> ParsingResult>;
type FromStringGenerator = Box<dyn Fn(String) -> Parser>;
type FromTokenGenerator = Box<dyn Fn(Token) -> Parser>;

fn boxed<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn parse_spaces(text: String) -> ParsingResult {
    for (i, x) in text.chars().enumerate() {
        if x != " ".chars().next().unwrap() {
            return Result::Ok(Node::new(
                Token::Number(i as i32),
                text[i..].to_string(),
            ));
        }
    }
    return Result::Err(());
}

fn parse_digit(text: String) -> ParsingResult {
    if text.len() > 0 {
        if let Ok(x) = text.chars().next().unwrap().to_string().parse::<i32>() {
            return Result::Ok(Node::new(
                Token::Number(x), 
                text[1..].to_string()
            ));
        }
    } 
    return Result::Err(());
}

fn gen_parse_char(x: char) -> Parser {
    boxed(move |text: String| -> ParsingResult {
        let mut it = text.chars();
        let text = it.next();
        if let Some(e) = text {
            Result::Ok(Node::new(Token::Word(e.to_string()), it.collect()))
        } else {
            Result::Err(())
        }
    })
}

fn gen_fin(x: Token) -> Parser {
    boxed(move |text: String| {
        let x = x.clone();
        if let Token::None = x {
            Result::Err(())
        } else {
            Result::Ok(Node::new(x, text))
        }
    })
}

fn bind(parse: Parser, gen: FromTokenGenerator) -> Parser {
    boxed(move |text| -> ParsingResult {
        let res = parse(text);

        if let Ok(x) = res {
            (gen(x.child))(x.tail)
        } else {
            Result::Err(())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spaces_test() {
        let text = "  1".to_string();
        let exp = Node::new(
            Token::Number(2),
            "1".to_string()
        );

        let act = parse_spaces(text);

        assert!(act.is_ok());
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn gen_parse_char_test() {
        let text = "123".to_string();
        let exp = Node::new(
            Token::Word("1".to_string()),
            "23".to_string()
        );

        let parser = gen_parse_char('1');
        let act = parser(text);

        assert!(act.is_ok());
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn parse_two_digit_test() {
        let text = "143".to_string();
        let exp = Node::new(Token::Number(14), "3".to_string());

        let parser = 
            bind(boxed(parse_digit), boxed(move |x| {
            bind(boxed(parse_digit), boxed(move |y| {
                if let Token::Number(x) = x && let Token::Number(y) = y {
                    gen_fin(Token::Number(x*10 + y))
                } else {
                    gen_fin(Token::None)
                }
            }))}));
        let act = parser(text);

        assert!(act.is_ok());
        assert_eq!(act.unwrap(), exp);
    }
}