
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

            if (Character.isDigit(c)) {
                result.push(String.valueOf(c));

                position ++;

                continue;
            }

            if (c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')') {

                if (stack.empty()) {
                    stack.push(String.valueOf(c));

                    position ++;

                    continue;
                }

                int a = 0;

                while (a < stack.size()) {
                    if (getPriority(String.valueOf(c)) <= getPriority(stack.get(a))) {
                        result.push(stack.pop());

                        a --;
                    } else {
                        stack.push(String.valueOf(c));

                        a ++;
                    }

                    a ++;
                }
            }

            position ++;
        }

        if (stack.empty() == false) {
            while (stack.size() != 0) {
                result.push(stack.pop());
            }
        }

        return result;
    }

    private static void visitor(Stack<String> stack) {

    }

    public static void main(String[] args) {
//        Stack<String> stringStack = parseProgram("1 + 2 * 3 - (4 + 5) / 6");

//        [ 1, 2, 3, *, +, 4, - ]
        Stack<String> stringStack = parseProgram("1 + 2 * 3 - 4");

        System.out.println(stringStack);

        visitor(stringStack);
    }
}