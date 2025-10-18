目标：
1. 掌握get_out_put_amount_and_remaining_accounts 的计算方法



内容：
1. 计算方法
For Zero-for-One (Token0 → Token1)
amount_out = L × (1/√P₁ - 1/√P₂)

For One-for-Zero (Token1 → Token0)
amount_out = L × (√P₂ - √P₁)


1) 实例
Scenario: Swap 1000 USDC for SOL
Current price: $100 USDC per SOL
Current liquidity: L = 1,000,000
Input: 1000 USDC
Direction: Zero-for-One (USDC → SOL)

Step 1: Calculate Price Impact
Current sqrt price: √100 = 10
Target sqrt price: √99 = 9.95 (example)

Step 2: Calculate Amount Out
amount_out = L × (1/√P₁ - 1/√P₂)
amount_out = 1,000,000 × (1/10 - 1/9.95)
amount_out = 1,000,000 × (0.1 - 0.1005)
amount_out = 1,000,000 × (-0.0005)
amount_out = -500 (negative means output)

Step 3: Apply Fees
fee_amount = input_amount × fee_rate
net_amount_out = amount_out - fee_amount