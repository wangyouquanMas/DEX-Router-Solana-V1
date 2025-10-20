目标：
1. 掌握log的用法
2. 理解Solana程序中的日志记录
3. 学习如何使用 msg! 宏和 Pubkey::log()


内容：
1. **log_swap_basic_info 函数的作用**
   - 记录代币交换操作的基本识别信息
   - 为调试和监控提供审计跟踪
   - 帮助追踪哪些代币和账户参与了交换

2. **日志记录的内容**
   - order_id: 订单的唯一标识符（仅当 > 0 时记录）
   - source_mint: 被交换的源代币地址
   - destination_mint: 目标代币地址
   - source_owner: 源代币账户的所有者
   - destination_owner: 目标代币账户的所有者

3. **Pubkey::log() 的工作原理**
   - 在Solana区块链上: 使用 sol_log_pubkey() 系统调用写入交易日志
   - 在测试环境中: 将公钥打印到标准输出

4. **运行测试**
   ```bash
   cd rust_tutorial/49.log/practice
   cargo test -- --nocapture
   ```

5. **在实际程序中查看日志**
   - Solana Explorer 中的交易日志
   - 使用 `solana logs` 命令
   - RPC getTransaction 响应中