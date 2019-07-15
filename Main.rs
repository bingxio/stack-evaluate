enum TokenType {
    TokenAdd,       // +
    TokenSubtract,  // -
    TokenMultiply,  // *
    TokenDivide,    // /
    TokenLParen,    //（
    TokenRParen,    // ）
    TokenNumber     // 0..9
}

fn token_type_string(token_type: TokenType) -> String {
    match token_type {
        TokenType::TokenAdd => String::from("TOKEN_ADD"),
        TokenType::TokenSubtract => String::from("TOKEN_SUBTRACT"),
        TokenType::TokenMultiply => String::from("TOKEN_MULTIPLY"),
        TokenType::TokenDivide => String::from("TOKEN_DIVIDE"),
        TokenType::TokenLParen => String::from("TOKEN_LPAREN"),
        TokenType::TokenRParen => String::from("TOKEN_RPAREN"),
        TokenType::TokenNumber => String::from("TOKEN_NUMBER")
    }
}

struct Token { literal: String, typedef: TokenType }

impl Token {
    fn new(literal: String, typedef: TokenType) -> Token {
        Token { literal, typedef }
    }

    fn show(self) -> String {
        format!("[ Token: literal = {}, typedef = {} ]",
                self.literal, token_type_string(self.typedef))
    }
}

fn main() {
    let tests = vec![
        "1 + 2 * 3 - 4"
    ];

    for x in tests {
        let tokens = parse_program(x);
    }
}

fn parse_program(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let a = src.chars().filter(|x| (x == &' '));

    println!("{:?}", a);

    return tokens
}
