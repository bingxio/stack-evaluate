
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
    let mut tests: Vec<String> = Vec::new();

    tests.push(String::from("1 + 2 * 3 - 4"));

    for x in tests {
        let tokens = parse_program(x);
    }
}

fn parse_program(mut src: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    src = src.replace(" ", "");

    let position = 0;

    while position < src.len() {
        tokens.push(
            Token {
                literal: src.get(position).to_string(),
                typedef: match src.get(position) {
                    '+' => TokenType::TokenAdd,
                    '-' => TokenType::TokenSubtract,
                    '*' => TokenType::TokenMultiply,
                    '/' => TokenType::TokenDivide,
                    '(' => TokenType::TokenLParen,
                    ')' => TokenType::TokenRParen,
                    '0'..'9' => TokenType::TokenNumber
                }
            }
        );

        position += 1;
    }

    return tokens
}
