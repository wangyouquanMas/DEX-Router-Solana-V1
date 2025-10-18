# 初始化 AmmConfig 阶段错误回顾

## 错误 1：Anchor 指令模块导出混乱
- **背景**：实现 `initialize_amm_config` 指令时，在 `lib.rs` 中重新导出指令上下文与处理函数。
- **错误**：在 `#[program]` 模块中直接引用 `instructions::initialize_amm_config`，导致 Anchor 生成的 `try_accounts` 等符号解析失败，引发 `unresolved import crate` / `expected type, found module` 编译错误。
- **原因**：Anchor 宏依赖 crate 根下的默认导出路径，直接引用子模块破坏了宏生成代码期望的命名空间。
- **方案**：明确导出策略——在 `instructions/mod.rs` 中保留 `pub use initialize_amm_config::*;`，并在 `lib.rs` 中通过 `use instructions::initialize_amm_config::InitializeAmmConfig;` 以及 `instructions::initialize_amm_config::initialize_amm_config(...)` 调用处理函数。

## 错误 2：手续费校验顺序导致断言失败
- **背景**：为 `initialize_amm_config` 添加单元测试，覆盖总费率超过上限的失败分支。
- **错误**：`validate_fee_inputs` 先校验单项费率，再校验总费率。当传入的 trade fee 等于分母时，优先触发 `InvalidTradeFeeRate`，与测试预期的 `TotalFeeRateTooHigh` 不符。
- **原因**：单项上限检查优先执行，导致在计算总费率前即抛出错误。
- **方案**：调整校验顺序，先计算并校验 `total_fee_rate` 是否超过 `FEE_RATE_DENOMINATOR`，再检查单项费率上限，确保超总额场景返回目标错误码。


## 错误 3：单元测试期望错误码不匹配
- **背景**：运行 `cargo test` 验证费用校验逻辑。
- **错误**：测试用例设定 `trade_fee_rate = FEE_RATE_DENOMINATOR`，在调整校验顺序前后均触发单项上限超出的错误，导致断言失败。
- **原因**：测试数据本身已违反单项约束，导致捕获到的错误码不是总费率分支。
- **方案**：在测试中构造总和溢出的场景，同时保证各单项在允许范围，例如 `trade_fee_rate = MAX_TRADE_FEE_RATE`, `protocol_fee_rate = MAX_PROTOCOL_FEE_RATE`, `fund_fee_rate = FEE_RATE_DENOMINATOR - (trade + protocol) + 1`，确保命中 `TotalFeeRateTooHigh` 分支。




背景：在本地 `fun-uniswap-v3` Anchor 项目中执行 `anchor test`，期望完成 Rust 单元测试与 TypeScript 集成测试。

错误1：
背景：【运行 `anchor test` 调用 Anchor.toml 中的测试脚本】
错误：【命令输出 `yarn: error: no such option: -p`，随后提示 `Parsing scenario file run`，发现系统中的 `yarn` 实际指向 Debian 的 `cmdtest`】
方案：【通过 `npm install --global yarn` 安装官方 Yarn CLI（或替换为 corepack 管理的版本），确保 `yarn run ts-mocha` 使用真实 Yarn；同时在 `Anchor.toml` 中使用 `yarn run ts-mocha -- -p ./tsconfig.json -t 1000000 tests/**/*.ts` 将参数转发给 `ts-mocha`】

错误2：
背景：【重新运行 `anchor test` 后进入 TypeScript 测试阶段】
错误：【Yarn 报错 `Command "ts-mocha" not found.`，因为 `node_modules/.bin/ts-mocha` 尚未生成】
方案：【在项目根目录执行 `yarn install`，安装 package.json 中定义的依赖，使 `ts-mocha` 等可执行文件进入 PATH】

错误3：
背景：【安装依赖时运行 `yarn install`】
错误：【安装过程中提示 `@solana/codecs-numbers@2.3.0` 需要 Node `>=20.18.0`，当前环境为 `18.19.1`】
方案：【使用 `npm install --global n && n 20.18.0`（或其他 Node 版本管理工具）升级 Node 至满足要求的版本，随后重新执行 `yarn install` 并再次运行 `anchor test`】

