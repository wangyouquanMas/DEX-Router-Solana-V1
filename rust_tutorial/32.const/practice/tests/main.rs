mod config{
    pub const TIMEOUT: u32 = 30;
    pub const SERVER_NAME: &str = "api.example.com";
}

#[test]
fn test_const(){
    println!("Max retries:{}", config::TIMEOUT);
}


