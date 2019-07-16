use std::collections::HashMap;

fn main() {
    let tests = vec![
        // 1 2 3 * + 4 -
        "1 + 2 * 3 - 4".to_string(),
        // 1 2 + 3 * 4 -
        "(1 + 2) * 3 - 4".to_string(),
        // 1 2 3 * + 4 5 + - 6 /
        "1 + 2 * 3 - (4 + 5) / 6".to_string()
    ];

    for i in tests {
        let stack = parse_program(i);

        for i in stack {
            print!("{} ", i);
        }

        println!("");
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
    OpNumber,   // 0..9
    OpReturn    // return
}

fn opcode_string(op: OpCode) -> String {
    String::from(match op {
        OpCode::OpAdd => "OP_ADD",
        OpCode::OpSubtract => "OP_SUBTRACT",
        OpCode::OpMultiply => "OP_MULTIPLY",
        OpCode::OpDivide => "OP_DIVIDE",
        OpCode::OpNumber => "OP_NUMBER",
        OpCode::OpReturn => "OP_RETURN"
    })
}

struct Chunk<'a> {
    opcode_stack: &'a mut Vec<OpCode>,
    values_stack: &'a mut Vec<i32>
}

impl Chunk<'_> {
    fn emit_constant(self, value: i32) {
        self.opcode_stack.push(OpCode::OpNumber);
        self.values_stack.push(value);
    }

    fn emit_opcode(self, opcode: OpCode) {
        self.opcode_stack.push(opcode);
    }

    fn show() {}
}

fn transform<'a>(stack: Vec<char>) -> Chunk<'a> {
    let mut a: Vec<OpCode> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    let chunk = Chunk {
        opcode_stack: &mut a,
        values_stack: &mut b
    };

    return chunk;
}
