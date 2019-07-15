
import java.lang.RuntimeException
import java.lang.StringBuilder
import java.util.*
import kotlin.collections.ArrayList
import kotlin.collections.HashMap

private fun main() {
    arrayListOf(
        "1 + 2 * 3 - (4 + 5) / 6",
        "1 + 2 * 3 - 4").forEach {

        val tokens = parseProgram(it)
        val stack = transformToStack(tokens)

        stack.toString()
    }
}

private enum class TokenType {
    TOKEN_ADD,      // +
    TOKEN_SUBTRACT, // -
    TOKEN_MULTIPLY, // *
    TOKEN_DIVIDE,   // /
    TOKEN_NUMBER,   // 0..9
    TOKEN_LPAREN,   // (
    TOKEN_RPAREN    // )
}

private data class Token(val tokenType: TokenType, val literal: String)

private fun parseProgram(src: String): ArrayList<Token> {
    val tokens = ArrayList<Token>()
    var position = 0

    val sourceReplaceWhitespace = src.filter { it != ' ' }

    while (position < sourceReplaceWhitespace.length) {
        tokens.add(
            when {
                sourceReplaceWhitespace[position] == '+' -> Token(TokenType.TOKEN_ADD, "+")
                sourceReplaceWhitespace[position] == '-' -> Token(TokenType.TOKEN_SUBTRACT, "-")
                sourceReplaceWhitespace[position] == '*' -> Token(TokenType.TOKEN_MULTIPLY, "*")
                sourceReplaceWhitespace[position] == '/' -> Token(TokenType.TOKEN_DIVIDE, "/")
                sourceReplaceWhitespace[position] == '(' -> Token(TokenType.TOKEN_LPAREN, "(")
                sourceReplaceWhitespace[position] == ')' -> Token(TokenType.TOKEN_RPAREN, ")")

                sourceReplaceWhitespace[position].toInt() in 48..57 ->
                    Token(TokenType.TOKEN_NUMBER, sourceReplaceWhitespace[position].toString())

                else -> throw RuntimeException("Unknown character: ${sourceReplaceWhitespace[position]}")
            }
        )

        position ++
    }

    return tokens
}

private enum class OpCode {
    OP_ADD,         // +
    OP_SUBTRACT,    // -
    OP_MULTIPLY,    // *
    OP_DIVIDE,      // /
    OP_LOCAL,       // 0..9
    OP_RETURN       // end
}

private class Chunk {
    val stack = Stack<OpCode>()
    val value = Stack<Int>()

    fun emitConstant(v: Int) {
        stack.push(OpCode.OP_LOCAL)
        value.push(v)
    }

    fun emitOpCode(opCode: OpCode) {
        stack.push(opCode)
    }

    fun emitOpCode(tokenType: TokenType) {
        stack.push(
            when (tokenType) {
                TokenType.TOKEN_ADD -> OpCode.OP_ADD
                TokenType.TOKEN_SUBTRACT -> OpCode.OP_SUBTRACT
                TokenType.TOKEN_MULTIPLY -> OpCode.OP_MULTIPLY
                TokenType.TOKEN_DIVIDE -> OpCode.OP_DIVIDE

                else -> { null }
            }
        )
    }

    override fun toString(): String {
        val stringBuilder = StringBuilder()
        var k = 0

        stack.forEach {
            print(it)

            if (it == OpCode.OP_LOCAL)
                System.out.format("%10d \n", value[k ++])
            else
                println()
        }

        println("==================")

        return stringBuilder.toString()
    }
}

private fun getPriority(tokenType: TokenType): Int {
    val map = HashMap<TokenType, Int>()

    map[TokenType.TOKEN_ADD] = 1;
    map[TokenType.TOKEN_SUBTRACT] = 1;
    map[TokenType.TOKEN_MULTIPLY] = 2;
    map[TokenType.TOKEN_DIVIDE] = 2;
    map[TokenType.TOKEN_LPAREN] = 3;
    map[TokenType.TOKEN_RPAREN] = 3;

    return map[tokenType]!!
}

private fun transformToStack(tokens: ArrayList<Token>): Chunk {
    val chunk = Chunk()

    val operator = Stack<TokenType>()

    tokens.forEach {
        when (it.tokenType) {
            TokenType.TOKEN_NUMBER -> chunk.emitConstant(it.literal.toInt())
            TokenType.TOKEN_LPAREN -> operator.push(it.tokenType)

            TokenType.TOKEN_RPAREN -> {

            }

            else -> when {
                operator.empty() -> chunk.emitOpCode(it.tokenType)

                getPriority(it.tokenType) <= getPriority(operator.peek()) -> {
                    while (!operator.empty() && getPriority(it.tokenType) <= getPriority(operator.peek()))
                        chunk.emitOpCode(operator.pop())
                    chunk.emitOpCode(it.tokenType)
                }

                else -> chunk.emitOpCode(it.tokenType)
            }
        }
    }

    chunk.emitOpCode(OpCode.OP_RETURN)

    return chunk
}