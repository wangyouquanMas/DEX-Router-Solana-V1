Papers:
1. For a broader overview (not a full paper but research-oriented), check this Medium article on pathfinding algorithms for DEX aggregation: 
https://medium.com/deeplink-labs/pathfinding-algorithms-for-dex-aggregation-and-smart-order-routing-9e9feaf6b796


2. Measuring DEX Efficiency and The Effect of an Enhanced Routing Method on Both DEX Efficiency and Stakeholders' Benefits
ntroduces Standardized Total Arbitrage Profit (STAP) via convex optimization to measure DEX efficiency. Compares a line-graph-based routing algorithm vs. DFS, showing improved trader returns, stable TVL, and benefits for liquidity providers. Based on Uniswap V2 data from mid-2024.
https://arxiv.org/pdf/2508.03217



3.A Line Graph-Based Framework for Identifying Optimal Routing Paths in Decentralized Exchanges
Proposes a novel line-graph-based algorithm for finding more profitable trading paths in DEXs like Uniswap V2, outperforming DFS in profitability and gas efficiency. Reduces arbitrage opportunities, enhancing overall market efficiency. Includes empirical comparisons on historical data
https://arxiv.org/pdf/2504.15809


4.An Efficient Algorithm for Optimal Routing Through Constant Function Market Makers
Presents an efficient algorithm for optimal routing in constant product market makers (e.g., Uniswap-style DEXs). Focuses on minimizing slippage and maximizing output in multi-hop trades via convex optimization. Theoretical and practical for aggregator routers.
https://angeris.github.io/papers/routing-algorithm.pdf



Codebases:
1.Core SOR engine for Uniswap V2/V3, implementing multi-hop pathfinding via graph algorithms (e.g., improved DFS with heuristics for profitability). Includes off-chain routing API for aggregators; supports slippage optimization and gas estimation. Referenced in Uniswap's 2021 open-source announcement  
https://github.com/Uniswap/smart-order-router

2.https://github.com/hummingbot/gateway

3.https://github.com/wangyouquanMas/Cobra#