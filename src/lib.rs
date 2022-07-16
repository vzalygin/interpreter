#![feature(let_chains)]

use std::any::type_name;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Node,
    Number(i32),
    Word(String),
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    children: Vec<Token>,
    tail: String,
}

impl Node {
    fn from_many(children: Vec<Token>, tail: String) -> Node {
        Node { children, tail }
    }

    fn from_one(child: Token, tail: String) -> Node {
        Node { 
            children: vec![child],
            tail
        }
    }
}

type ParsingResult = Result<Node, ()>;
type Parser = Box<dyn Fn(String) -> ParsingResult>;
type FromStringGenerator = Box<dyn Fn(String) -> Parser>;
type FromTokenGenerator = Box<dyn Fn(Vec<Token>) -> Parser>;

fn boxed<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn parse_spaces(text: String) -> ParsingResult {
    for (i, x) in text.chars().enumerate() {
        if x != " ".chars().next().unwrap() {
            return Result::Ok(Node::from_one(
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
            return Result::Ok(Node::from_one(
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
            Result::Ok(Node::from_one(Token::Word(e.to_string()), it.collect()))
        } else {
            Result::Err(())
        }
    })
}

fn gen_trivial(x: String) -> Parser {
    boxed(move |text: String| {
        let x = x.clone();
        Result::Ok(Node::from_one(Token::Word(x), text))
    })
}

fn bind(parse: Parser, gen: FromTokenGenerator) -> Parser {
    boxed(move |text| -> ParsingResult {
        let res = parse(text);

        if let Ok(x) = res {
            (gen(x.children))(x.tail)
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
        let exp = Node::from_many(
            vec![Token::Number(2)],
            "1".to_string()
        );

        let act = parse_spaces(text);

        assert!(act.is_ok());
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn gen_parse_char_test() {
        let text = "123".to_string();
        let exp = Node::from_many(
            vec![Token::Word("1".to_string())],
            "23".to_string()
        );

        let parser = gen_parse_char('1');
        let act = parser(text);

        assert!(act.is_ok());
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn parse_two_digit_test() {
        let text = "123".to_string();
        let exp = Node::from_one(Token::Number(12), "3".to_string());

        let parser = 
            bind(boxed(parse_digit), boxed(|x| {
            bind(boxed(parse_digit), boxed(|y| {
                let x = x[0];
                let y = y[0];
                if let Token::Number(x) = x && let Token::Number(y) = y {
                    Result::Ok(Token::Number(x*10 + y))   
                } else {
                    Result::Err(())
                }
            }))}));
    }
}