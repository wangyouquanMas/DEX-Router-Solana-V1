目标：
1. 掌握sha256算法使用

内容：
1. 计算sha256算法
    // 1. 拼接字符串
    let preimage = format!("{}:{}", namespace, name); // 例如 "account:AmmConfig"
    // 2. 计算 SHA256 哈希
    let hash = sha2::Sha256::digest(preimage.as_bytes());
