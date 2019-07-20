use std::{env, fs::File, io::prelude::*, iter, collections};

enum OpCode {
    OpLoadConst,
    OpStore,
    OpBinaryAdd,
    OpBinarySubtract,
    OpBinaryMultiply,
    OpBinaryDivide,
    OpPrint,
    OpReturn
}

impl OpCode {
    fn string(opcode: &OpCode) -> &'static str {
        match opcode {
            OpCode::OpLoadConst      => "OpLoadConst",
            OpCode::OpStore          => "OpStore",
            OpCode::OpBinaryAdd      => "OpBinaryAdd",
            OpCode::OpBinarySubtract => "OpBinarySubtract",
            OpCode::OpBinaryMultiply => "OpBinaryMultiply",
            OpCode::OpBinaryDivide   => "OpBinaryDivide",
            OpCode::OpPrint          => "OpPrint",
            OpCode::OpReturn         => "OpReturn"
        }
    }

    fn is_opcode(a: &String) -> (bool, OpCode) {
        let a = a.as_str();

        for i in vec![
            OpCode::OpLoadConst,
            OpCode::OpStore,
            OpCode::OpBinaryAdd,
            OpCode::OpBinarySubtract,
            OpCode::OpBinaryMultiply,
            OpCode::OpBinaryDivide,
            OpCode::OpPrint,
            OpCode::OpReturn
        ] {
            if a == OpCode::string(&i) {
                return (true, i);
            }
        }

        return (false, OpCode::OpReturn);
    }
}

struct Parser<'a> {
    chars: iter::Peekable<std::str::Chars<'a>>,
    code_stack: Vec<OpCode>,
    data_stack: Vec<String>,
    exec_stack: Vec<f32>,
    enviro_map: collections::HashMap<String, f32>
}

impl<'a> Parser<'a> {
    fn is_whitespace(&self, char_int: i32) -> bool {
        char_int == 32 || char_int == 10
    }

    fn is_identifier(&self, char_int: i32) -> bool {
        (char_int >= 65 && char_int <= 90) || char_int == 95 || (char_int >= 97 && char_int <= 122)
    }

    fn parse(&mut self) {
        loop {
            match self.chars.next() {
                Some(character) => {
                    if self.is_whitespace(character as i32) {
                        self.parse_skip_whitespace();
                    } else if self.is_identifier(character as i32) {
                        self.parse_opcode(character);
                    } else {
                        self.data_stack.push(character.to_string());
                    }
                },
                None => break
            }
        }
    }

    fn parse_skip_whitespace(&mut self) {
        while let Some(&character) = self.chars.peek() {
            if self.is_whitespace(character as i32) == false {
                break;
            } else {
                self.chars.next();
            }
        }
    }

    fn parse_opcode(&mut self, advance: char) {
        let mut code = String::new();

        code.push(advance);

        while let Some(&character) = self.chars.peek() {
            if self.is_identifier(character as i32) {
                code.push(character);
                self.chars.next();
            } else {
                break;
            }
        }

        if OpCode::is_opcode(&code).0 == false {
            self.data_stack.push(code);
        } else {
            self.code_stack.push(
                OpCode::is_opcode(&code).1
            );
        }
    }

    fn execute(&mut self) {
        let (mut position, mut k) = (0, 0);

        while let bytecode = self.code_stack.get(position).unwrap() {
            match bytecode {
                OpCode::OpLoadConst => {
                    let a = self.data_stack.get(k).unwrap();
                    let a: f32 = a.parse().unwrap();

                    self.exec_stack.push(a);

                    k += 1;
                },

                OpCode::OpStore => {
                    let a = self.data_stack.get(k).unwrap();
                    let b = self.exec_stack.last().unwrap();

                    self.enviro_map.insert(a.clone(), *b);

                    k += 1;
                }

                OpCode::OpPrint => println!("{}", self.exec_stack.last().unwrap()),

                OpCode::OpReturn => break,

                _ => {
                    let a = self.exec_stack.pop().unwrap();
                    let b = self.exec_stack.pop().unwrap();

                    match bytecode {
                        OpCode::OpBinaryAdd => self.exec_stack.push(b + a),

                        _ => unimplemented!()
                    }
                }
            }

            position += 1;
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut parser = Parser {
        chars: contents.chars().peekable(),
        code_stack: Vec::new(),
        data_stack: Vec::new(),
        exec_stack: Vec::new(),
        enviro_map: collections::HashMap::new()
    };

    parser.parse();
    parser.execute();
}