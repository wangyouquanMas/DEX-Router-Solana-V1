### Step 1 Guidance: 设计 `AmmConfig`

- **理解参考实现**
  - `raydium-clmm/programs/amm/src/states/config.rs` 给出完整例子。重点字段：`owner`、`protocol_fee_rate`、`trade_fee_rate`、`tick_spacing`、`fund_fee_rate`、`fund_owner` 等。
  - 常量：`AMM_CONFIG_SEED`（PDA 种子），`FEE_RATE_DENOMINATOR_VALUE`（费率分母）。
  - 空间：`LEN` 表示账号所需字节，含 8 字节账户判别符。

- **你的任务**
  1. **确定字段**  
     - 最小集合：`bump`、`authority`/`owner`、费率配置、`tick_spacing`。可先精简字段集合，后续迭代扩展。
     - 审核类型：费率常用 `u32`（单位 10^-6）；`tick_spacing` 用 `u16`；地址字段用 `Pubkey`。
  2. **定义种子与常量**  
     - 在 `state/mod.rs` 或新文件（例如 `state/amm_config.rs`）声明：
       ```
       pub const AMM_CONFIG_SEED: &str = "amm-config";
       pub const FEE_RATE_DENOMINATOR: u32 = 1_000_000;
       ```
     - 种子用于 `Pubkey::find_program_address`.
  3. **编写结构体**  
     - 使用 `#[account]` 注解，`#[derive(Default, Debug)]` 方便测试。
     - 计算空间：`LEN = 8 + ...`，将所有字段字节长度相加。
  4. **辅助方法**  
     - 添加验证方法（如 `validate_fee_rates`) 以集中逻辑；可参考 Raydium 的 `is_authorized`.
  5. **组织代码**  
     - 推荐在 `state/mod.rs` 引入子模块 `pub mod amm_config;`；在 `state/amm_config.rs` 放置详细实现，保持清晰。
     - `state/mod.rs` 中 `pub use amm_config::*;` 方便外部引用。

- **顺序建议**
  - 在 `state/amm_config.rs` 写结构体与常量 → 修改 `state/mod.rs` 暴露内容 → （可选）在 `lib.rs` `pub use state::AmmConfig;` → `anchor build` 验证编译。

完成后即可进入 Step2，基于 `AmmConfig` 结构体编写初始化指令。