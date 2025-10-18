
目标：
1. 掌握getTokenAccountsByOwner rpc方法的用法


内容：
1. 用法
返回指定代币所有者的所有 SPL Token 账户。

1） 输入
curl https://api.devnet.solana.com -s -X \
  POST -H "Content-Type: application/json" -d ' 
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getTokenAccountsByOwner",
    "params": [
      "A1TMhSGzQxMr1TboBKtgixKz1sS6REASMxPo1qsyTSJd",
      {
        "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
      },
      {
        "commitment": "finalized",
        "encoding": "jsonParsed"
      }
    ]
  }
'

2） 输出
{
  "jsonrpc": "2.0",
  "result": {
    "context": { "apiVersion": "2.0.15", "slot": 341197933 },
    "value": [
      {
        "pubkey": "BGocb4GEpbTFm8UFV2VsDSaBXHELPfAXrvd4vtt8QWrA",
        "account": {
          "data": {
            "program": "spl-token",
            "parsed": {
              "info": {
                "isNative": false,
                "mint": "2cHr7QS3xfuSV8wdxo3ztuF4xbiarF6Nrgx3qpx3HzXR",
                "owner": "A1TMhSGzQxMr1TboBKtgixKz1sS6REASMxPo1qsyTSJd",
                "state": "initialized",
                "tokenAmount": {
                  "amount": "420000000000000",
                  "decimals": 6,
                  "uiAmount": 420000000.0,
                  "uiAmountString": "420000000"
                }
              },
              "type": "account"
            },
            "space": 165
          },
          "executable": false,
          "lamports": 2039280,
          "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "rentEpoch": 18446744073709551615,
          "space": 165
        }
      }
    ]
  },
  "id": 1
}

参考：
1.直接可执行：https://solana.com/zh/developers/cookbook/tokens/get-all-token-accounts
2.使用文档：https://solana.com/zh/docs/rpc/http/gettokenaccountsbyowner
返回Token account ; Token mint; Wallet


