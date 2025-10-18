// 子模块示例 - 展示子模块的概念和好处

// 主模块：电商系统
pub struct EcommerceSystem {
    pub name: String,
    pub version: String,
}

impl EcommerceSystem {
    pub fn new(name: &str) -> Self {
        EcommerceSystem {
            name: name.to_string(),
            version: "1.0".to_string(),
        }
    }
    
    pub fn get_info(&self) {
        println!("电商系统: {} v{}", self.name, self.version);
    }
}

pub mod user_management{
    use super::EcommerceSystem;
    
    #[derive(Debug)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
        pub role: UserRole,
    }
    
    #[derive(Debug)]
    pub enum UserRole {
        Customer,
        Seller,
        Admin,
    }
    
    impl User {
        pub fn new(id: u32, name: String, email: String, role: UserRole) -> Self {
            User { id, name, email, role }
        }
        
        pub fn get_info(&self) {
            println!("用户: {} ({})", self.name, self.email);
        }
    }
    
    // 子模块的内部函数
    fn validate_email(email: &str) -> bool {
        email.contains("@")
    }
    
    pub fn create_user(id: u32, name: String, email: String, role: UserRole) -> Option<User> {
        if validate_email(&email) {
            Some(User::new(id, name, email, role))
        } else {
            None
        }
    }
    
    // 子模块的嵌套子模块
    pub mod authentication {
        use super::User;
        
        pub fn login(user: &User, password: &str) -> bool {
            // 简化的登录逻辑
            password.len() >= 6
        }
        
        pub fn logout(user: &User) {
            println!("用户 {} 已登出", user.name);
        }
        
        // 内部子模块
        mod security {
            pub fn hash_password(password: &str) -> String {
                format!("hashed_{}", password)
            }
        }
    }
}


//演示子模块的使用
pub fn demonstrate_submodules(){
    println!("=== 子模块详细演示 ===");

    //1. 用户管理子模块
    println!("1. 用户管理子模块");
    let user = user_management::create_user(
        1,
        "张三".to_string(),
        "zhangsan@example.com".to_string(),
        user_management::UserRole::Customer,
    ).unwrap();
    user.get_info();


    //2. 使用嵌套子模块
    if user_management::authentication::login(&user,"password123"){
        println!("登录成功")
    }
}