目标：搭建好项目环境


1. 部署合约
2. 确保测试客户端可用，方便后续调试


参考内容：
# CLMM 合约部署与客户端交互指南

##  视频参考
**部署参考视频**: https://meeting.tencent.com/crm/2aMBELrv86  
**访问密码**: web3

补充：视频的#[cfg(feature = "devnet")] 有误，查看提示6.
---

##  目录
- [任务一：部署CLMM合约 + 编译客户端代码](#任务一部署clmm合约--编译客户端代码)
- [任务二：与合约进行交互](#任务二与合约进行交互)

---

## 任务一：部署CLMM合约 + 编译客户端代码

###  合约仓库
部署合约地址: https://github.com/raydium-io/raydium-clmm

按照仓库要求，先配置好环境，在编译前，参考下述提示。

###  重要配置提示

#### 1. 环境要求
- **推荐系统**: Ubuntu 24 (Linux 环境)
- **原因**: 防止 anchor 编译报错

#### 2. 更新 proc-macro2 依赖
** 文件位置**: `/root/raydium-cpi/raydium-amm-v3/programs/amm/Cargo.toml`

在 `dependencies` 部分添加:
```toml
proc-macro2 = "1.0.95"
```

#### 3. Anchor 版本配置
修改 anchor 版本为 0.31.1 以保持一致，修改网络为devnet:

** 文件 1**: `/root/raydium-cpi/raydium-amm-v3/Anchor.toml`
```toml
anchor_version = "0.31.1"

[provider]
cluster = "devnet"
```


** 文件 2**: `/root/raydium-cpi/raydium-amm-v3/programs/amm/Cargo.toml`
```toml
anchor-lang = { version = "0.31.1", features = ["init-if-needed"] }
anchor-spl = { version = "0.31.1", features = ["metadata", "memo"] }
```

#### 4. 网络配置 (DevNet)
默认节点是本地 - 需要改为 devnet:

**查看当前配置:**
```bash
solana config get
```

**示例输出:**
```
Config File: /home/ubuntu/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900 
Keypair Path: /home/ubuntu/.config/solana/id.json 
Commitment: confirmed 
```

**修改配置:**
```bash
solana config set --url https://api.devnet.solana.com
```

**预期输出:**
```
Config File: /home/ubuntu/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com 
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /home/ubuntu/.config/solana/id.json 
Commitment: confirmed 
```

#### 5. 设置合约管理员账户
将合约的管理员账户设置为你的钱包账户，为后续创建池子做准备。

** 文件位置**: `raydium-amm-v3/programs/amm/src/lib.rs`

```rust
pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("3xbCoRgPcuUhUdsVJHrq79gmcGUT3VwqrHgMTkV296cP");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("3xbCoRgPcuUhUdsVJHrq79gmcGUT3VwqrHgMTkV296cP");
}
```

> **注意**: 将 pubkey 替换为你的钱包账户地址

#### 6. 使用 DevNet 特性编译

先指定你要创建的合约地址
#[cfg(feature = "devnet")]
declare_id!("你指定的合约地址【Program ID】");

如何指定Program ID ：执行 solana-keygen new --outfile target/deploy/raydium_v3-keypair.json --no-bip39-passphrase
执行完后，会创建一个keypair，作为ProgramID.  注意--outfile 输出目录地址要对


**为什么要指定ProgramID？**: 合约会验证部署的地址与声明的地址是否一致。不匹配会导致合约调用失败。


接着执行下面的命令

**构建命令:**
```bash
anchor build -- --features devnet 
```

编译完后，执行
anchor keys sync

如果发现ProgramID发生变化，那么要重新执行
anchor build -- --features devnet 



#### 7. 资金要求
**所需 SOL**: 约 9 个 SOL 用于合约部署

**获取 DevNet 代币**: https://faucet.solana.com/
- **限制**: 每次请求 5 SOL [需要自己选]
- **最大**: 连续 2 次请求 (总计 10 SOL)

**钱包设置命令:**
```bash
# 创建新钱包
solana-keygen new

# 查看钱包地址
solana address

# 查看余额
solana balance
```

#### 7.1  拓展：合约清理 (SOL 回收)
```bash
solana program close <合约地址> --recipient <钱包地址> --bypass-warning
```


#### 7.2 部署合约
anchor deploy --program-name raydium_amm_v3 --program-keypair target/deploy/raydium_v3-k
eypair.json   
由于网络问题，第一次部署可能会失败。重复执行上述命令，就可以成功部署了


###  客户端代码编译

#### 步骤 1: 配置客户端
**文件**: `client_config.ini` (在合约根目录)

**需要修改的内容:**
- `http_url`, `ws_url` → devnet 环境 URL
- `payer_path`, `admin_path` → 你的钱包地址路径
- `raydium_v3_Program` → 你部署的合约程序 ID

#### 步骤 2: 构建客户端
```bash
cd client
cargo build --release
```

**成功标志**: 编译好的 release 文件出现在 `target` 目录中。

---

## 任务二：与合约进行交互

基于之前学习的内容，与合约进行交互。确保准备好所有必需的账户。

###  命令参考
```bash
./target/release/client --help
```

###  代币管理

#### 创建代币铸造账户
```bash
./target/release/client new-mint --decimals 6
```
spl-token create-token

> 创建精度为 6 位小数的代币铸造账户

如果使用命令行输出当前创建的spl token ? 
spl-token accounts


如何使用命令行输出当前的token account ?

1. 如何将sol 转为 wsol 
 ./target/release/client wrap-sol 5000000000


#### 创建代币账户
```bash
./target/release/client new-token <MINT_A_ADDRESS> <YOUR_WALLET>
```
spl-token create-account token_mint_address

#### 铸造代币
```bash
./target/release/client mint-to <MINT_A_ADDRESS> <TOKEN_ACCOUNT_A> 1000000000000
```
spl-token mint 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 100 Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt

> **注意**: 代币铸造单位转换: `1 SPL 代币 = 10^6 单位`  
> 浏览器显示使用 SPL 代币作为基本单位

###  池子配置

#### 创建池子配置账户
```bash
./target/release/client create-config <CONFIG_INDEX> <TICK_SPACING> <TRADE_FEE_RATE> <PROTOCOL_FEE_RATE> <FUND_FEE_RATE>
```

**示例:**
```bash
./target/release/client create-config 1 60 3000 1200 0
```
> 配置 CLMM 池子的元数据属性 (交易手续费等)

**验证配置:**
```bash
./target/release/client p-config 1
```

#### 创建池子
```bash
./target/release/client create-pool <CONFIG_INDEX> <INITIAL_PRICE> <MINT_A_ADDRESS> <MINT_B_ADDRESS>
```
注意：要在配置文件里面配置池子，代码里走的是配置文件，没走命令行参数
修改地址： /root/raydium-cpi/raydium-amm-v3/client_config.ini
[Pool]  将mint0,mint1改为你上面创建好的两个mint
mint0 = 6TPhwEKEqKGsPskmp4SzrkZ24q3aDu98UfwQ5ipQaFZv
mint1 = D94yRtXXFWoXfaQZ8pDeN8J6Lme1DwaJW6ug8hkCiEMr

注意：这两个token mint 的token account 必须先创建好。

**示例:**
```bash
./target/release/client create-pool 1 1.0 6TPhwEKEqKGsPskmp4SzrkZ24q3aDu98UfwQ5ipQaFZv D94yRtXXFWoXfaQZ8pDeN8J6Lme1DwaJW6ug8hkCiEMr
```
> 使用 AMM 配置 1 创建初始价格为 1.0 的池子

###  流动性管理

#### 添加区间流动性
```bash
./target/release/client open-position [OPTIONS] <TICK_LOWER_PRICE> <TICK_UPPER_PRICE> <INPUT_AMOUNT>
```

**示例:**
```bash
./target/release/client open-position 0.8 1.2 --is-base-0 1000000000 --with-metadata
```
> 设置价格区间 0.8 - 1.2，存入 100 个 tokenA

###  代币交换

#### 执行交换
```bash
./target/release/client swap <TOKEN_ACCOUNT_A> <TOKEN_ACCOUNT_B> --base-in <AMOUNT>
```

**示例:**
```bash
./target/release/client swap 7frMp7KuSeZaRUZDXV2mch4fP1wrfADYy8Q3vXJ9bFYG EAedwuyTGXwGFsDkDxsG6hJvhCNciXWb5jQ97uu77MxC --base-in 10000000
```
> 用 10 个 token mint A 兑换 token mint B

---

## 🏁 总结
本指南涵盖了从 CLMM 合约部署到客户端交互的完整过程。请仔细按照每个步骤操作，确保在进行下一阶段之前满足所有先决条件。

