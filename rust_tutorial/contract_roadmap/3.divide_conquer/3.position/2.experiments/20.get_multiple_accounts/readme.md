目标：
1. 掌握get_multiple_accounts用法

内容：
1. 用法
返回一组 Pubkeys 的账户信息。

1）输入
curl https://api.devnet.solana.com -s -X \
  POST -H "Content-Type: application/json" -d ' 
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getMultipleAccounts",
    "params": [
      [
        "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
        "4fYNw3dojWmQ4dXtSGE9epjRGy9pFSx62YypT7avPYvA"
      ],
      {
        "encoding": "base58",
        "commitment": "finalized"
      }
    ]
  }
'


2）输出
{
  "jsonrpc": "2.0",
  "result": {
    "context": { "apiVersion": "2.0.15", "slot": 341197247 },
    "value": [
      {
        "data": ["", "base58"],
        "executable": false,
        "lamports": 88849814690250,
        "owner": "11111111111111111111111111111111",
        "rentEpoch": 18446744073709551615,
        "space": 0
      },
      {
        "data": ["", "base58"],
        "executable": false,
        "lamports": 998763433,
        "owner": "2WRuhE4GJFoE23DYzp2ij6ZnuQ8p9mJeU6gDgfsjR4or",
        "rentEpoch": 18446744073709551615,
        "space": 0
      }
    ]
  },
  "id": 1
}

参考文档：
https://solana.com/zh/docs/rpc/http/getmultipleaccounts