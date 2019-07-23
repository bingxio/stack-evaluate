use std::{env, fs::File, io, iter, collections};

#[derive(Debug)]
enum OpCode {
    OpLoadConst,        // load a const value to data stack.
    OpStore,            // store a value to global environment.
    OpBinaryAdd,        // operator of add.
    OpBinarySub,        // operator of subtract.
    OpBinaryMul,        // operator of multiply.
    OpBinaryDiv,        // operator of divide.
    OpCompareLess,      // operator of less than.
    OpCompareGreater,   // operator of greater than.
    OpJumpIfFalse,      // if condition is false to jump else branch.
    OpPrint,            // display stack top value.
    OpReturn            // break program.
}

impl OpCode {
    fn is_opcode(a: &String) -> (bool, OpCode) {
        for i in vec![
            OpCode::OpLoadConst,
            OpCode::OpStore,
            OpCode::OpBinaryAdd,
            OpCode::OpBinarySub,
            OpCode::OpBinaryMul,
            OpCode::OpBinaryDiv,
            OpCode::OpCompareLess,
            OpCode::OpCompareGreater,
            OpCode::OpJumpIfFalse,
            OpCode::OpPrint,
            OpCode::OpReturn
        ] {
            if *a == format!("{:?}", i) {
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
    env_map: collections::HashMap<String, f32>
}

impl<'a> Parser<'a> {
    fn is_whitespace(&self, char_int: i32) -> bool {
        char_int == 32 || char_int == 10
    }

    fn is_identifier(&self, char_int: i32) -> bool {
        (char_int >= 65 && char_int <= 90) || char_int == 95 || (char_int >= 97 && char_int <= 122)
    }

    fn is_digit(&self, char_int: i32) -> bool {
        char_int >= 48 && char_int <= 57
    }

    fn parse(&mut self) {
        loop {
            match self.chars.next() {
                Some(character) => {
                    if self.is_whitespace(character as i32) {
                        self.parse_skip_whitespace();
                    } else if self.is_identifier(character as i32) {
                        self.parse_opcode(character);
                    } else if self.is_digit(character as i32) {
                        self.parse_digit(character);
                    }
                }
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

        if OpCode::is_opcode(&code).0 != false {
            self.code_stack.push(
                OpCode::is_opcode(&code).1
            );
        } else {
            self.data_stack.push(code);
        }
    }

    fn parse_digit(&mut self, advance: char) {
        let mut digit = String::new();

        digit.push(advance);

        while let Some(&character) = self.chars.peek() {
            if self.is_digit(character as i32) {
                digit.push(character);
                self.chars.next();
            } else {
                break;
            }
        }

        self.data_stack.push(digit);
    }

    fn execute(&mut self) {
        let (mut position, mut k) = (0, 0);

        while let Some(bytecode) = self.code_stack.get(position) {
            match bytecode {
                OpCode::OpLoadConst => {
                    let a = self.data_stack.get(k).unwrap();
                    let a: f32 = a.parse().unwrap();

                    self.exec_stack.push(a);

                    println!("{:?}", self.exec_stack);

                    k += 1;
                }

                OpCode::OpStore => {
                    let a = self.data_stack.get(k).unwrap();
                    let b = self.exec_stack.last().unwrap();

                    self.env_map.insert(a.clone(), *b);

                    k += 1;
                }

                OpCode::OpJumpIfFalse => {
                    position = self.data_stack.get(k).unwrap().parse().unwrap();
                    k += 1;

                    continue;
                }

                OpCode::OpPrint => println!("{:.6}", self.exec_stack.last().unwrap()),

                OpCode::OpReturn => break,

                _ => {
                    let a = self.exec_stack.pop().unwrap();
                    let b = self.exec_stack.pop().unwrap();

                    match bytecode {
                        OpCode::OpBinaryAdd => self.exec_stack.push(b + a),
                        OpCode::OpBinarySub => self.exec_stack.push(b - a),
                        OpCode::OpBinaryMul => self.exec_stack.push(b * a),
                        OpCode::OpBinaryDiv => self.exec_stack.push(b / a),

                        OpCode::OpCompareLess => self.exec_stack.push(
                            if b < a { 1. } else { 0. }
                        ),
                        OpCode::OpCompareGreater => self.exec_stack.push(
                            if b > a { 1. } else { 0. }
                        ),

                        _ => unimplemented!()
                    }
                }
            }

            position += 1;
        }
    }
}

fn main() {
    let mut contents = String::new();

    if env::args().len() == 2 {
        use io::prelude::*;

        File::open(
            env::args().nth(1).expect("Could not found test file path.")
        ).unwrap().read_to_string(&mut contents).unwrap();
    } else {
        use io::{Write};

        loop {
            let mut input = String::new();

            print!(">>> ");

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Cannot read line !");

            if input == "OpReturn\n" {
                break;
            } else {
                contents.push_str(&input);
            }
        }
    }

    let mut parser = Parser {
        chars: contents.chars().peekable(),
        code_stack: Vec::new(),
        data_stack: Vec::new(),
        exec_stack: Vec::new(),
        env_map: collections::HashMap::new()
    };

    parser.parse();
    parser.execute();
}