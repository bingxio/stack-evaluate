

[In my blog](https://blog.xyiio.cn/2019/07/07/2019-07-07/)

```
1 + 2 * 3 - 4 -> 1 2 3 * + 4 - 
OP_LOCAL         1
OP_LOCAL         2
OP_LOCAL         3
OP_MULTIPLY
OP_ADD
OP_LOCAL         4
OP_SUBTRACT
OP_RETURN
3.000000
(1 + 2) * 3 - 4 -> 1 2 + 3 * 4 - 
OP_LOCAL         1
OP_LOCAL         2
OP_ADD
OP_LOCAL         3
OP_MULTIPLY
OP_LOCAL         4
OP_SUBTRACT
OP_RETURN
5.000000
1 + 2 * 3 - (4 + 5) / 6 -> 1 2 3 * + 4 5 + 6 / - 
OP_LOCAL         1
OP_LOCAL         2
OP_LOCAL         3
OP_MULTIPLY
OP_ADD
OP_LOCAL         4
OP_LOCAL         5
OP_ADD
OP_LOCAL         6
OP_DIVIDE
OP_SUBTRACT
OP_RETURN
5.500000
7 / 9 -> 7 9 / 
OP_LOCAL         7
OP_LOCAL         9
OP_DIVIDE
OP_RETURN
0.777778
```
