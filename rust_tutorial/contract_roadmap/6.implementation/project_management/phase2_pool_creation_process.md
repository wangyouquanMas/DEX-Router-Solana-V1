# Phase 2 · Pool Creation 流程整理

## 1. 背景概述
- **目标**：在 Raydium CLMM 框架下完成初始池的创建，使后续流动性头寸、交易、奖励等逻辑具备统一的池级状态来源。
- **参考资料**：
  - `contract_roadmap/3.divide_conquer/2.create_pool` 系列文档（交易拆解、账户推导、实验记录）。
  - `contract_roadmap/6.implementation/1.anchor_tutorial/raydium-clmm` 源码，特别是 `programs/amm/src/instructions/create_pool.rs` 与 `states/pool.rs`。
- **关键输出**：`PoolState` 账户、金库账户、观察账户、TickArray 位图账户，以及 `PoolCreatedEvent` 事件。

## 2. 前置条件与输入参数
- **配置依赖**：
  - 已初始化的 `AmmConfig`（决定 `tick_spacing`、费用率等）。
  - 已验证的 Token Mint（`token_mint_0 < token_mint_1` 的排序约束）。
- **调用参数**：
  - `sqrt_price_x64`：以 Q64.64 表示的初始价格（来源见 `experiments/4.bitmap_account_key_generation` 中的价格推导）。
  - `open_time`：允许交易的起始时间（当前实现中只需保证 `Clock::unix_timestamp > open_time`）。
- **费用支付者**：`pool_creator` 负责支付所有 PDA 初始化费用并作为签名者。

## 3. 账户体系与 PDA 推导
- **PoolState**：`PDA = ["pool", amm_config, token_mint_0, token_mint_1]`，结构体定义见 `states/pool.rs`。
- **Token Vaults**：
  - `token_vault_0`: `["pool_vault", pool_state, token_mint_0]`
  - `token_vault_1`: `["pool_vault", pool_state, token_mint_1]`
- **ObservationState**：`["observation", pool_state]`，用于 TWAP 与价格历史。
- **TickArrayBitmap**：`["pool_tick_array_bitmap_extension", pool_state]`，追踪 TickArray 初始化情况。
- **可选 SupportMint PDA**：如果 mint 属于 Token-2022 且需要白名单校验，会通过 `remaining_accounts` 传入。

> 这些 PDA 生成逻辑可在 `experiments/3.pool_account_key_generation`、`concepts/2.derive_account.md` 中找到逐步推导示例。

## 4. 客户端交易组装（Go/TS SDK 案例）
1. **Compute Budget 指令**：
   - `setComputeUnitPrice` & `setComputeUnitLimit`，具体字节布局参考 `2.core_components.md`。
2. **CreatePool 指令数据**：
   - 8 字节指令 discriminator：`{233,146,209,142,207,104,64,188}`。
   - 16 字节 `sqrt_price_x64`（大端编码）。
   - 8 字节 `open_time`（小端编码）。
   - 账户按严格顺序填写，详见 `concepts/2.derive_account.md` 中的表格。
3. **交易签名并发送**：`pool_creator` 作为唯一签名者；在 `1.create_transaction.md` 中可找到 CLI 调用示例及链上观测链接。

## 5. On-chain 指令流程（`create_pool.rs`）
1. **白名单/支持性校验**：
   - `support_mint_associated_is_initialized`、`is_supported_mint` 检查 Token-2022 扩展与白名单。
2. **时间约束**：`Clock::unix_timestamp` 必须大于 `open_time`，否则失败。
3. **PoolState 初始化**：
   - `pool_state.load_init()` 返回可写引用；计算初始 `tick`：`tick_math::get_tick_at_sqrt_price`。
4. **金库账户创建**：
   - 调用 `create_token_vault_account`（见 `util/token.rs`），内部自动调用：
     - `get_account_data_size` 动态获取账户空间。
     - `create_or_allocate_account` 创建/分配 PDA。
     - 可选 `initialize_immutable_owner`。
     - `initialize_account3` 将 `pool_state` 设为 Token 账户 authority。
5. **ObservationState 初始化**：
   - `observation_state.load_init()?.initialize(pool_id)`，结构参考 `concepts/1.observationState.md`。
6. **PoolState 字段赋值**：
   - `PoolState::initialize` 设置 mint 信息、vault、tick spacing、liquidity、fee growth、状态位、奖励数组等。
7. **TickArrayBitmap 初始化**：
   - `tick_array_bitmap.load_init()?.initialize(pool_id)`。
8. **事件发射**：
   - `emit!(PoolCreatedEvent { ... })`，记录关键字段供前端监听。

## 6. 关键状态字段说明
- `sqrt_price_x64` 与 `tick_current`：初始价格及对应 tick，保证数学库后续计算一致。
- `liquidity`：初始为 0，待流动性添加后更新。
- `fee_growth_global_*`：全局费增长计数，初始化为 0。
- `reward_infos`：三组奖励槽位，默认 `RewardInfo::new(pool_creator)`。
- `tick_array_bitmap`：默认全 0，用于标记已初始化的 TickArray。
- `status`：位标志控制 Swap/流动性操作开关，初始为启用状态。

## 7. 错误与约束
- `TokenMintMismatch` / `NotSupportMint`：Token 排序或白名单失败。
- `InvalidTickSpacing`：`AmmConfig.tick_spacing` 与输入参数不符。
- `PoolAlreadyInitialized`：重复初始化同一 `(amm_config, mint0, mint1)` 组合。
- `Clock` 相关错误：`open_time` 未满足，将触发 `require_gt!` 失败。

> 在 `error.rs` 中可找到完整错误定义；在扩展实现时需对新错误进行文档标注。

## 8. 测试与验证建议
- **Rust 单测**：
  - 使用 `solana_program_test` 模拟上下文，覆盖创建成功、重复创建、mint 排序错误、白名单缺失等场景。
- **TypeScript 集成测试**：
  - 参考 `tests/fun-uniswap-v3.ts`，增补创建池流程并断言池状态字段。
- **链上验证**：
  - 利用 `experiments/1.pool_info/practice` 脚本读取池账户，确认状态字段与事件。

## 9. 常见问题排查
- **WSOL 资金准备**：使用 `./target/release/client wrap-sol <amount>` 先将 SOL 包装为 WSOL（参见 `1.create_transaction.md`）。
- **TickArray 超界**：若初始化 Tick 超出默认 bitmap 范围，需要同时初始化 `TickArrayBitmapExtension`（相关逻辑在 `states/pool.rs` 内详注）。
- **Token-2022 支持**：确保金库存证扩展正确初始化；如需不可变 owner，则必须传入扩展类型并调用 `initialize_immutable_owner`。

## 10. 完成标志
- `create_pool` 指令能够成功执行并生成期望账户。
- `PoolCreatedEvent` 在日志中可见，`PoolState` 字段与输入参数匹配。
- Rust/TS 测试覆盖关键路径且通过。
- 文档与 Roadmap 状态同步更新，记录命令与排障经验。


