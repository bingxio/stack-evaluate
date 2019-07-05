import java.util.HashMap;
import java.util.Map;
import java.util.Stack;

public class Main {

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

            if (Character.isLetterOrDigit(c)) {
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
                    int a = stack.size() - 1;

                    while (!stack.get(a).equals("(")) {
                        System.out.println(stack.get(a));

                        a --;
                    }

                    position ++;

                    continue;
                }

                if (getPriority(String.valueOf(c)) <= getPriority(stack.peek())) {
                    int a = stack.size() - 1;

                    while (getPriority(String.valueOf(c)) <= getPriority(stack.get(a)) && !stack.get(a).equals("(")) {
                        result.push(stack.pop());

                        if (--a == -1)
                            break;
                    }

                    stack.push(String.valueOf(c));
                } else {
                    stack.push(String.valueOf(c));
                }
            }

            position ++;
        }

        while (stack.empty() == false) {
            result.push(stack.pop());
        }

        return result;
    }

    private static void visitor(Stack<String> stack) {
        Stack<Integer> integerStack = new Stack<>();

        int a, b, position = 0;

        while (position < stack.size()) {
            String str = stack.get(position);

            if (Character.isDigit(str.charAt(0))) {
                integerStack.push(Integer.valueOf(str));

                position ++;

                continue;
            }

            switch (str) {
                case "+":
                    b = integerStack.pop();
                    a = integerStack.pop();

                    integerStack.push(a + b);

                    break;
                case "-":
                    b = integerStack.pop();
                    a = integerStack.pop();

                    integerStack.push(a - b);

                    break;
                case "*":
                    b = integerStack.pop();
                    a = integerStack.pop();

                    integerStack.push(a * b);

                    break;
                case "/":
                    b = integerStack.pop();
                    a = integerStack.pop();

                    integerStack.push(a / b);

                    break;
            }

            position ++;
        }

        System.out.println(integerStack.peek());
    }

    public static void main(String[] args) {
//        Stack<String> stringStack = parseProgram("1 + 2 * 3 - (4 + 5) / 6");

//        [ 1, 2, 3, *, +, 4, - ]
//        Stack<String> stringStack = parseProgram("1 + 2 * 3 - 4");

//        [ 6, 3, 2, +, *, 5, + ]
        Stack<String> stringStack = parseProgram("6 * (3 + 2) + 5");

//        [ 6, 3, *, 2, + ]
//        Stack<String> stringStack = parseProgram("6 * 3 + 2");

        System.out.println(stringStack);

        visitor(stringStack);
    }
}