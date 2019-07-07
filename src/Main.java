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
        Stack<OpCode> opCodeStack = new Stack<>();
        Stack<Integer> integerStack = new Stack<>();

        void emitConstant(int value) {
            opCodeStack.add(OpCode.OP_LOCAL);
            integerStack.add(value);
        }

        void emitOpCode(OpCode opCode) {
            opCodeStack.add(opCode);
        }

        void debug() {
            for (int i = 0, k = 0; i < opCodeStack.size(); i ++) {
                System.out.print(opCodeStack.get(i));

                if (opCodeStack.get(i).equals(OpCode.OP_LOCAL))
                    System.out.format("%10d \n", integerStack.get(k ++));
                else
                    System.out.println("");
            }
        }
    }

    private static Stack<String> parseProgram(String source) {
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

        int position = 0;

        while (position < stringStack.size()) {
            switch (stringStack.get(position)) {
                case "+":
                    chunk.emitOpCode(OpCode.OP_ADD);

                    position ++;

                    continue;
                case "-":
                    chunk.emitOpCode(OpCode.OP_SUBTRACT);

                    position ++;

                    continue;
                case "*":
                    chunk.emitOpCode(OpCode.OP_MULTIPLY);

                    position ++;

                    continue;
                case "/":
                    chunk.emitOpCode(OpCode.OP_DIVIDE);

                    position ++;

                    continue;
            }

            chunk.emitConstant(Integer.valueOf(stringStack.get(position)));

            position ++;
        }

        chunk.emitOpCode(OpCode.OP_RETURN);

        return chunk;
    }

    private static void visitor(Chunk chunk) {
        int position = 0, k = 0;

        Stack<Double> stack = new Stack<>();

        double a, b;

        while (position < chunk.opCodeStack.size()) {
            switch (chunk.opCodeStack.get(position)) {
                case OP_LOCAL:
                    stack.push(Double.valueOf(chunk.integerStack.get(k ++)));

                    break;
                case OP_ADD:
                    b = stack.pop();
                    a = stack.pop();

                    stack.push(a + b);

                    break;
                case OP_SUBTRACT:
                    b = stack.pop();
                    a = stack.pop();

                    stack.push( a - b);

                    break;
                case OP_MULTIPLY:
                    b = stack.pop();
                    a = stack.pop();

                    stack.push(a * b);

                    break;
                case OP_DIVIDE:
                    b = stack.pop();
                    a = stack.pop();

                    stack.push(a / b);

                    break;
                case OP_RETURN:
                    System.out.println(stack.peek());

                    break;
            }

            position ++;
        }
    }

    public static void main(String[] args) {

//        [ 1, 2, 3, *, +, 4, 5, +, -, 6, / ]
//
//        OP_LOCAL      1
//        OP_LOCAL      2
//        OP_LOCAL      3
//        OP_MULTIPLY
//        OP_ADD
//        OP_LOCAL      4
//        OP_LOCAL      5
//        OP_ADD
//        OP_SUBTRACT
//        OP_LOCAL      6
//        OP_DIVIDE
//        OP_RETURN
        Stack<String> stringStack1 = parseProgram("1 + 2 * 3 - (4 + 5) / 6");

//        [ 1, 2, 3, *, +, 4, - ]
//
//        OP_LOCAL      1
//        OP_LOCAL      2
//        OP_LOCAL      3
//        OP_MULTIPLY
//        OP_ADD
//        OP_LOCAL      4
//        OP_SUBTRACT
//        OP_RETURN
        Stack<String> stringStack2 = parseProgram("1 + 2 * 3 - 4");

//        [ 6, 3, 2, +, *, 5, / ]
//
//        OP_LOCAL      6
//        OP_LOCAL      3
//        OP_LOCAL      2
//        OP_ADD
//        OP_MULTIPLY
//        OP_LOCAL      5
//        OP_DIVIDE
//        OP_RETURN
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
//
//        OP_LOCAL      1
//        OP_LOCAL      2
//        OP_LOCAL      3
//        OP_ADD
//        OP_LOCAL      4
//        OP_MULTIPLY
//        OP_ADD
//        OP_LOCAL      5
//        OP_SUBTRACT
//        OP_RETURN
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

            chunk.debug();

            visitor(chunk);

            System.out.println("==================");
        }
    }
}