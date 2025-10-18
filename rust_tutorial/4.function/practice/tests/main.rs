struct AccountInfo{
    is_writable: bool,
}

impl AccountInfo{
    // 使用 self 返回当前类型 AccountInfo 的实例
    fn new(is_writable: bool) -> Self{
        Self{ is_writable}
    }
}

#[test]
fn test_self(){
    let account_info = AccountInfo::new(true);
    println!("Account writable: {}", account_info.is_writable);
}

