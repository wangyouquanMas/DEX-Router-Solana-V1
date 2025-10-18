# Phase 1 AmmConfig 执行计划

1：任务目标：【设计并实现 AmmConfig 初始化逻辑，奠定全局 AMM 配置基础，包括费用档位、tick 间距等核心参数，确保后续池与头寸建立有明确约束】
2：执行步骤：
step1: 在 `state` 模块设计 `AmmConfig` 结构体，定义关键字段（如 `authority`, `fee_tier_bps`, `tick_spacing`, `protocol_fee_rate` 等），并确定 PDA 种子与空间预算。

step2: 在 `instructions` 模块创建 `initialize_amm_config.rs`，编写 Anchor 上下文结构体，声明所需账户（`payer`, `authority`, `amm_config`, `system_program` 等）。

step3: 实现 `initialize_amm_config` 处理函数，包含参数校验（范围检查、去重）、账户初始化约束和事件触发。

step4: 在 `errors` 模块定义与 AmmConfig 相关的错误码（如 `InvalidFeeTier`, `AlreadyInitialized`）。

step5: 为 `initialize_amm_config` 增加 Anchor 事件（可选），记录成功初始化的配置参数。

step6: 在 Rust 侧添加 `#[cfg(test)]` 单元测试（使用 `solana_program_test` 或 Anchor 的 `ProgramTest`），覆盖成功与失败场景。

step7: 在 `tests/fun-uniswap-v3.ts` 或新增 TS 测试文件，编写集成测试调用 `initializeAmmConfig` 指令，验证初始化与重复初始化错误。

step8: 更新 `1.anchor_tutorial/readme.md` 与 `project_management/1.roadmap.md`，记录实现状态与测试命令；如遇问题追加至 `error_logs/Anchor测试环境问题排查.md`。

step9: 执行 `anchor build` 与 `anchor test` 验证新功能；完成后 `git add`、`git commit -m "错误总结"`、`git push`。

3：任务完成标准：【AmmConfig 账户结构与初始化指令实现且通过 Anchor 构建与测试；Rust/TS 双侧测试覆盖成功退回场景；文档与日志更新同步完成】


