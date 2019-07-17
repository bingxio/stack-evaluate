use std::collections::HashMap;
use std::cmp::Ordering;
use std::any::Any;

fn main() {
    let tests = vec![
        // 1 2 3 * + 4 -
        "1 + 2 * 3 - 4".to_string(),
        // 1 2 + 3 * 4 -
//        "(1 + 2) * 3 - 4".to_string(),
        // 1 2 3 * + 4 5 + - 6 /
//        "1 + 2 * 3 - (4 + 5) / 6".to_string()
    ];

    for i in tests {
        let stack = parse_program(i);

        for i in stack.clone() {
            print!("{} ", i);
        }

        println!();

        let chunk = transform(stack);

        chunk.show();
    }
}

fn get_priority(i: char) -> Option<u8> {
    let mut map: HashMap<char, u8> = HashMap::new();

    map.insert('+', 1);
    map.insert('-', 1);
    map.insert('*', 2);
    map.insert('/', 2);
    map.insert('(', 3);
    map.insert(')', 3);

    return map.get(&i).cloned();
}

fn parse_program(mut source: String) -> Vec<char> {
    let mut backup: Vec<char> = Vec::new();
    let mut result: Vec<char> = Vec::new();

    source = source.replace(" ", "");

    for i in source.chars() {
        match i {
            '0'..='9' => result.push(i),

            '(' => backup.push(i),

            ')' => loop {
                let a = backup.pop().unwrap();

                if a == '(' || backup.is_empty() {
                    break;
                } else {
                    result.push(a);
                }
            }

            _ => {
                let mut a = backup.len() as isize - 1;

                while a != -1 && get_priority(i) <= get_priority(backup[a as usize]) {
                    if backup[a as usize] != '(' {
                        result.push(
                            backup.pop().unwrap()
                        );
                    } else {
                        break;
                    }

                    a -= 1;
                }

                backup.push(i);
            }
        }
    }

    while backup.is_empty() == false {
        result.push(
            backup.pop().unwrap()
        );
    }

    return result;
}

enum OpCode {
    OpAdd,      // +
    OpSubtract, // -
    OpMultiply, // *
    OpDivide,   // /
    OpLocal,   // 0..9
    OpReturn    // return
}

fn opcode_string(op: &OpCode) -> String {
    String::from(match op {
        OpCode::OpAdd => "OP_ADD",
        OpCode::OpSubtract => "OP_SUBTRACT",
        OpCode::OpMultiply => "OP_MULTIPLY",
        OpCode::OpDivide => "OP_DIVIDE",
        OpCode::OpLocal => "OP_LOCAL",
        OpCode::OpReturn => "OP_RETURN"
    })
}

struct Chunk {
    opcode_stack: Vec<OpCode>,
    values_stack: Vec<i32>
}

impl Chunk {
    fn emit_constant(&mut self, value: i32) {
        self.opcode_stack.push(OpCode::OpLocal);
        self.values_stack.push(value);
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        self.opcode_stack.push(opcode);
    }

    fn show(self) {
        let mut k = 0;

        for i in self.opcode_stack.iter() {
            print!("{}", opcode_string(i));

            match i.type_id().cmp(&OpCode::OpLocal.type_id()) {
                Ordering::Equal => {
                    println!("     {}", self.values_stack.get(k).unwrap());
                    k += 1;
                },

                _ => println!()
            }
        }
    }
}

fn transform(stack: Vec<char>) -> Chunk {
    let a: Vec<OpCode> = Vec::new();
    let b: Vec<i32> = Vec::new();

    let mut chunk = Chunk {
        opcode_stack: a,
        values_stack: b
    };

    for i in stack {
        match i {
            '0'..='9' => chunk.emit_constant(i as i32),

            '+' => chunk.emit_opcode(OpCode::OpAdd),
            '-' => chunk.emit_opcode(OpCode::OpSubtract),
            '*' => chunk.emit_opcode(OpCode::OpMultiply),
            '/' => chunk.emit_opcode(OpCode::OpDivide),

            _ => unimplemented!()
        }
    }

    chunk.emit_opcode(OpCode::OpReturn);

    return chunk;
}
