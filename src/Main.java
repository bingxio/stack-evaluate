import java.util.*;

public class Main {

    enum OpCode {
        OP_LOCAL,
        OP_ADD,
        OP_SUBTRACT,
        OP_MULTIPLY,
        OP_DIVIDE,
        OP_RETURN
    }

    private static int getPriority(String s) {
        Map<String, Integer> map = new HashMap<>();

        map.put("+", 1);
        map.put("-", 1);
        map.put("*", 2);
        map.put("/", 2);
        map.put("(", 3);
        map.put(")", 3);

        return map.get(s);
    }

    private static class Chunk {
        ArrayList<OpCode> opCodeArrayList = new ArrayList<>();
        ArrayList<Integer> integerArrayList = new ArrayList<>();

        void emitConstant(int value) {
            opCodeArrayList.add(OpCode.OP_LOCAL);
            integerArrayList.add(value);
        }

        void emitOpCode(OpCode opCode) {
            opCodeArrayList.add(opCode);
        }
    }

    private static Stack<String> parseProgram(@SuppressWarnings("SameParameterValue") String source) {
        Stack<String> result = new Stack<>();
        Stack<String> stack = new Stack<>();

        int position = 0;

        while (position < source.length()) {
            char c = source.charAt(position);

            if (Character.isSpaceChar(c)) {
                position ++;

                continue;
            }

            if (Character.isDigit(c)) {
                result.push(String.valueOf(c));

                position ++;

                continue;
            }

            if (c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')') {
                if (stack.empty() || c == '(') {
                    stack.push(String.valueOf(c));

                    position ++;

                    continue;
                }

                if (c == ')') {
                    while (stack.empty() == false && stack.peek().equals("(") == false)
                        result.push(stack.pop());

                    position ++;

                    continue;
                }

                if (getPriority(String.valueOf(c)) <= getPriority(stack.peek())) {
                    int a = stack.size() - 1;

                    while (getPriority(String.valueOf(c)) <= getPriority(stack.get(a))) {
                        if (stack.get(a).equals("(") == false)
                            result.push(stack.pop());
                        else {
                            stack.pop();

                            break;
                        }

                        if (--a == -1)
                            break;
                    }

                    stack.push(String.valueOf(c));
                } else
                    stack.push(String.valueOf(c));
            }

            position ++;
        }

        while (stack.empty() == false)
            result.push(stack.pop());

        return result;
    }

    private static Chunk transform(Stack<String> stringStack) {
        Chunk chunk = new Chunk();



        return chunk;
    }

    public static void main(String[] args) {

//        [ 1, 2, 3, *, +, 4, 5, +, -, 6, / ]
        Stack<String> stringStack1 = parseProgram("1 + 2 * 3 - (4 + 5) / 6");

//        [ 1, 2, 3, *, +, 4, - ]
        Stack<String> stringStack2 = parseProgram("1 + 2 * 3 - 4");

//        [ 6, 3, 2, +, *, 5, / ]
        Stack<String> stringStack3 = parseProgram("6 * (3 + 2) / 5");

//        [ 6, 3, *, 2, + ]
//
//        OP_LOCAL      6
//        OP_LOCAL      3
//        OP_MULTIPLY
//        OP_LOCAL      2
//        OP_ADD
//        OP_RETURN
        Stack<String> stringStack4 = parseProgram("6 * 3 + 2");

//        [ 1, 2, 3, +, 4, *, +, 5, - ]
        Stack<String> stringStack5 = parseProgram("1 + ((2 + 3) * 4) - 5");

        ArrayList<Stack<String>> stackArrayList = new ArrayList<>();

        stackArrayList.add(stringStack1);
        stackArrayList.add(stringStack2);
        stackArrayList.add(stringStack3);
        stackArrayList.add(stringStack4);
        stackArrayList.add(stringStack5);

        for (int i = 0; i < 5; i ++) {
            Stack<String> stringStack = stackArrayList.get(i);

            Chunk chunk = transform(stringStack);
        }
    }
}