fn main() {
    let tests = vec![
        // 1 2 3 * + 4 -
        "1 + 2 * 3 - 4".to_string(),
        // 1 2 + 3 * 4 -
        "(1 + 2) * 3 - 4".to_string(),
        // 1 2 3 * + 4 5 + - 6 /
        "1 + 2 * 3 - (4 + 5) / 6".to_string(),
        // 7 9 /
        "7 / 9".to_string()
    ];

    for i in tests {
        print!("{} -> ", i);

        // parse source expression to tokens.
        let stack = parse_program(i);

        for i in stack.clone() {
            print!("{} ", i);
        }

        println!();

        // transform tokens to data and opcode chunk.
        let chunk = transform(stack);

        chunk.display();

        // visit expression to get target value.
        visitor(chunk);
    }
}

fn get_priority(i: char) -> Option<u8> {
    use std::collections::HashMap;

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
    OpLocal,    // 0..9
    OpReturn    // return
}

fn opcode_string(op: &OpCode) -> &'static str {
    match op {
        OpCode::OpAdd => "OP_ADD",
        OpCode::OpSubtract => "OP_SUBTRACT",
        OpCode::OpMultiply => "OP_MULTIPLY",
        OpCode::OpDivide => "OP_DIVIDE",
        OpCode::OpLocal => "OP_LOCAL",
        OpCode::OpReturn => "OP_RETURN"
    }
}

struct Chunk {
    opcode_stack: Vec<OpCode>,
    values_stack: Vec<i32>
}

trait ChunkImpl {
    // emit a OP_LOCAL and some value to chunk.
    fn emit_constant(&mut self, value: i32);
    // only emit a opcode.
    fn emit_opcode(&mut self, opcode: OpCode);
    // display opcodes and values.
    // display value if it is OP_LOCAL else only opcode.
    fn display(&self);
}

impl ChunkImpl for Chunk {
    fn emit_constant(&mut self, value: i32) {
        self.opcode_stack.push(OpCode::OpLocal);
        self.values_stack.push(value);
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        self.opcode_stack.push(opcode);
    }

    fn display(&self) {
        let mut k = 0;

        for i in self.opcode_stack.iter() {
            print!("{}", opcode_string(i));

            if opcode_string(i) == opcode_string(&OpCode::OpLocal) {
                println!("{:>10}", self.values_stack.get(k).unwrap());
                k += 1;
            } else {
                println!();
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
            '0'..='9' => chunk.emit_constant(
                (i as i32) - 48
            ),

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

fn visitor(chunk: Chunk) {
    let mut stack: Vec<f32> = Vec::new();
    
    let mut k = 0;

    for i in chunk.opcode_stack {
        match i {
            OpCode::OpLocal => {
                stack.push(
                    *chunk.values_stack.get(k).unwrap() as f32
                );
                k += 1;
            }

            OpCode::OpReturn => break,

            _ => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                match i {
                    OpCode::OpAdd => stack.push(b + a),
                    OpCode::OpSubtract => stack.push(b - a),
                    OpCode::OpMultiply => stack.push(b * a),
                    OpCode::OpDivide => stack.push(b / a),

                    _ => unimplemented!()
                }
            }
        }
    }

    println!("{:.6}", stack.last().unwrap());
}