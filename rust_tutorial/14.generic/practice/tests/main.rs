
#[derive(Clone)]
struct AccountInfo{
    holder_name: String,
    balance: f64,
}

struct Account<T>{
    info: AccountInfo,
    account: T,
}

impl<T> Account<T>{
    //创建一个新的 Account 实例
    fn new(info: AccountInfo, account: T) -> Account<T>{
        Account{ info, account}
    }

    fn display_info(&self){
        println!("Account holder: {}",self.info.holder_name);
        println!("Balance: {}",self.info.balance);
    }
}

#[test]
fn test_generic(){
    let account_info = AccountInfo{
        holder_name: String::from("Alice"),
        balance: 1000.0,
    };

    //创建一个存储整数账户的 Account 实例
    let integer_account = Account::new(account_info.clone(), 12345);
    integer_account.display_info();

    //创建一个存储字符串账户的 Account 实例
    let string_account = Account::new(account_info,String::from("user_account"));
    string_account.display_info();
}