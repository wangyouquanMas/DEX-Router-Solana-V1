# Phase 2 Pool Lifecycle 执行计划

1：任务目标：【构建 Raydium CLMM 池生命周期基础能力：完成 `PoolState` 账户定义、`create_pool` 指令实现以及观察/错误/事件链路，确保后续流动性头寸、交易等功能拥有可复用且安全的池级核心逻辑】

2：执行步骤：
step1: 研读 `contract_roadmap/3.divide_conquer/2.create_pool` 中交易流程及账户明细，结合 `raydium-clmm` 源码梳理池初始化所需的 PDA、账户依赖与约束。
step2: 在 `programs/fun-uniswap-v3/src/state` 下新增 `pool_state.rs`，定义 `PoolState` 结构体字段（`amm_config`, `token_mint_0/1`, `token_vault_0/1`, `observation_state`, `tick_array_bitmap`, `authority`, `liquidity`, `sqrt_price_x64`, `protocol_fee_rate` 等）并实现 `space()` 及 PDA 种子常量。
step3: 更新 `state/mod.rs` 公开 `PoolState`，并补充与 `AmmConfig` 的关联常量或辅助函数（如 tick 间距、vault bump 校验）。
step4: 在 `instructions` 目录创建 `create_pool.rs`，编写 Anchor `Context<CreatePool>` 结构体，声明并标注所需账户：`payer`, `authority`, `amm_config`, `pool_state`, `token_mint_0/1`, `token_vault_0/1`, `observation_state`, `tick_array_bitmap`, `rent`, `token_program`, `system_program` 等，确保 mut/read 权限正确。
step5: 在 `create_pool` 处理函数中实现账户初始化流程：使用 `init`/`init_if_needed` 及 PDA bumps 创建池、金库和观察账户，设置初值（`sqrt_price_x64`, `tick_current`, `liquidity`, `fee_growth_global`），并拉起必要的 `token::initialize_account3` 调用。
step6: 实做参数与账户校验：校验 `amm_config` 匹配、token mint 顺序一致、mint decimals 相等限制、重复初始化防护、权威签名校验，将逻辑拆分至 `utils/validation.rs` 或独立验证函数。
step7: 在 `errors` 模块新增池相关错误（如 `PoolAlreadyInitialized`, `TokenMintMismatch`, `InvalidTickSpacing`, `VaultBumpMismatch`），并在 `events` 模块定义 `PoolCreated` 事件记录关键字段（配置索引、vault 地址、sqrt_price、authority 等）。
step8: 更新 `lib.rs` 注册 `CreatePool` 指令接口，确保模块导入、事件触发及返回值一致，必要时调整 `utils` 目录以共享 sqrt 价格与 tick 计算函数。
step9: 编写 Rust 单元测试：在 `state` 与 `instructions` 的 `#[cfg(test)]` 中使用 `solana_program_test` 构建上下文，覆盖成功创建、重复创建、mint 不匹配、权限错误等场景。
step10: 扩展 TypeScript 集成测试（`tests/fun-uniswap-v3.ts` 或新增 `create-pool.spec.ts`），模拟真实交易创建池，核验池账户布局和事件日志；同步记录命令行脚本（参考 `client/create_pool`）。
step11: 更新文档：在 `1.anchor_tutorial/readme.md` 说明新指令使用方式，在 `project_management/1.roadmap.md` 标记阶段进度，并在 `error_logs` 中记录调试要点。
step12: 执行 `anchor build` 与 `anchor test`，确保构建及测试通过；完成后运行 `git status`、`git add`、`git commit -m "错误总结"`、`git push` 提交远程。

3：任务完成标准：【`PoolState` 账户与 `create_pool` 指令在 Rust/TS 测试中通过验证；事件与错误定义覆盖关键分支；相关文档同步更新；本地 `anchor build`、`anchor test` 均成功，且代码已推送远程仓库】


### What’s Next?

1. **Rust Tests (Step 9)**  
   - Add `solana_program_test` coverage for `create_pool`: happy path, duplicate init, mint order mismatch, authority mismatch. Start with integration tests under `tests/` or `#[cfg(test)]` modules around the instruction.

2. **Validation & Error Guardrails**  
   - Enforce any remaining checks: open-time vs. clock, mint allowlists, duplicate pool prevention, vault bump sanity. Tie these into the error codes we added.

3. **Instruction Wiring & Remaining Accounts**  
   - Ensure future instructions (e.g., add liquidity, swap) receive the bitmap extension via remaining accounts. Confirm event payloads cover everything downstream consumers need.

4. **Documentation & TS Tests**  
   - Update docs/roadmaps to reflect the new flow. Extend or add TypeScript tests once Rust coverage is in place.

We can start with the Rust tests if you’re ready; just let me know how you’d like to proceed.

