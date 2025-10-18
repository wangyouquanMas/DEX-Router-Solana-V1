use std::ops::{Deref, DerefMut};

struct Account<T>{
    info: AccountInfo,
    account: T,
}

struct AccountInfo{
    is_writable: bool,
}

impl AccountInfo{
    fn new(is_writable: bool) -> Self{
        Self{ is_writable}
    }
}


#[derive(Debug)]
struct AmmConfig{
    owner: String,
    index: u16,
    trade_fee_rate: u32,
}

impl AmmConfig{
    fn new() -> Self{
        Self{
            owner: String::new(),
            index: 0,
            trade_fee_rate:0,
        }
    }
}


impl<T> Deref for Account<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.account
    }
}


impl<T> DerefMut for Account<T>{
    fn  deref_mut(&mut self) -> &mut Self::Target{
        //安全检查： 确保账户可用
        if !self.info.is_writable{
            panic!("The given Account is not mutable");
        }
        &mut self.account
    }
}


#[test]
fn demonstrate_deref_mut(){
    println!("--- deref_mut() 演示 ---");

    //创建可写的账户
    let mut writable_account = Account{
        info: AccountInfo::new(true), //科协
        account: AmmConfig::new(),
    };

    //使用 deref_mut() 获取可变引用
    let amm_config = writable_account.deref_mut();

    //直接给字段赋值
    amm_config.owner = "Alice".to_string();
    amm_config.index = 1;
    amm_config.trade_fee_rate = 3000;

    println!(" 赋值后： {:?}", amm_config);
    println!(" 账户数据：{:?}", writable_account.account);

}



fn main(){
    demonstrate_deref_mut();
}