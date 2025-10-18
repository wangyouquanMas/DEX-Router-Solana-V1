目标：
1. 掌握换底公式
2. 掌握change of base 在 get tick at sqrt price中的应用

内容：
1. 公式
log_a(x) = log_b(x)/log_b(a)


2. 应用
We have:
Input: A price ratio in base-2 logarithm (log₂(price))
Target: Need to convert to base-√1.0001 logarithm (log_√1.0001(price))
Change of base formula: log_√1.0001(price) = log₂(price) / log₂(√1.0001)

2.1 为什么要换底？ 

𝑝(𝑖) = 1.0001^i

sqrt(p(i)) = sqrt(1.0001)^i  [代码中]

=> i = log base(sqrt(1.0001))(sqrt(p)) 