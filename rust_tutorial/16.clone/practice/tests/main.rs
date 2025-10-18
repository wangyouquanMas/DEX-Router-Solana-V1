use std::ptr;

#[derive(Clone)]  // 使得 AccountInfo 可以克隆
struct AccountInfo {
    holder_name: String,
    balance: f64,
}

struct Account<T> {
    info: AccountInfo,
    account: T,
}

impl<T> Account<T> {
    // 创建一个新的 Account 实例
    fn new(info: AccountInfo, account: T) -> Account<T> {
        Account { info, account }
    }

    // 显示账户信息
    fn display_info(&self) {
        println!("Account holder: {}", self.info.holder_name);
        println!("Balance: {}", self.info.balance);
    }
}

#[test]
fn test_generic() {
    let account_info = AccountInfo {
        holder_name: String::from("Alice"),
        balance: 1000.0,
    };
    
    // 打印拷贝前的内存位置
    println!("Before clone, account_info address: {:?}", ptr::addr_of!(account_info));

    // 创建一个存储整数账户的 Account 实例
    let integer_account = Account::new(account_info.clone(), 12345);  // 克隆 AccountInfo
    integer_account.display_info();


    // 打印拷贝后的内存位置
    println!("After clone, account_info address: {:?}", ptr::addr_of!(integer_account.info));

    // 创建一个存储字符串账户的 Account 实例
    let string_account = Account::new(account_info, String::from("user_account"));  // 直接使用 account_info
    string_account.display_info();
}