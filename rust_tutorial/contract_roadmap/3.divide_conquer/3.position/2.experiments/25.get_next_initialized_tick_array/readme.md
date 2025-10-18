目标：
1. 输出池子信息

  const pool = await program.account.poolState.fetch(new web3.PublicKey(POOL_ID));
  console.log({
    ammConfig: pool.ammConfig.toBase58(),
    tokenMint0: pool.tokenMint0.toBase58(),
    tokenMint1: pool.tokenMint1.toBase58(),
    tickSpacing: pool.tickSpacing,
    liquidity: pool.liquidity.toString(),
    sqrtPriceX64: pool.sqrtPriceX64.toString(),
    tickCurrent: pool.tickCurrent,
    status: pool.status,
    rewardInfos: pool.rewardInfos, // array of 3
  });