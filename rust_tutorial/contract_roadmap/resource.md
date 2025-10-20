
uniswap-v3-book:
https://rareskills.io/uniswap-v3-book

https://github.com/raydium-io/raydium-docs/tree/master/dev-resources

https://uniswapv3book.com/

https://app.uniswap.org/whitepaper.pdf
https://app.uniswap.org/whitepaper-v3.pdf
https://app.uniswap.org/whitepaper-v4.pdf

https://github.com/Jeiwan/uniswapv3-book

https://www.smartcontract.engineer/courses


面试题：
uniswapv2 flashloan是怎么实现的？
v3和v2相比有什么改进，除了集中流动性，还有什么不同？
v3的tick是什么，跨tick交易说说细节？
curve的预言机有研究吗，compound 清算逻辑，compound 如何判断用户是否能借贷，市场价格波动剧烈产生坏账怎么办？
compound利率怎么计算的？
如何计算string占用多少slot，你们用的可升级是什么，为什么不用uups用transparent，他们各有什么优缺点
钻石代理了解吗？
delegatecall给空地址会出现什么情况？
delegatecall后selfdestruct会怎样？

evm
抽象合约和接口的区别/状态变量修饰符有几种/ 动态数组可以用memory吗/重载和重写
代理合约介绍
无常损失
第一次添加池子的Lp怎么算，为什么在计算的时候减一个值
uniswapv3 交易过程，以及流动性不足的时候交易会报什么错误
闪存（uniswapv4）
抽象账户和7702
三种call介绍、消耗的gas
transfer、send、callvalue 三种方式的区别和消耗的gas
Receive 和fallback区别
mev是什么，有什么好处吗
blob了解吗
说一下uniswapv2和v3的原理，然后出计算题，怎么推出兑换的数量
uniswapv3的ticks、tickmap的作用，怎么计算下一个tick
uniswapv3tick 区间密度是怎么分布的
evm 交易类型有哪些：type 0 type1 等等
evm上的安全问题了解哪些，重入攻击怎么解决

solana
接入一个dex的时候，需要拿到什么 （地址以及 accountmeta等）
ata和pda的区别
solana的堆栈问题是如何解决的
solana 的交易长度有限，存入地址过多，可以用什么办法解决
solana的测试工具了解哪些