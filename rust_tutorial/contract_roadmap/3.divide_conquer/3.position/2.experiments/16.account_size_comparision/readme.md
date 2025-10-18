目标:
1. 掌握account 比较


内容：
1. 比较
if mint0 > mint1 {
       std::mem::swap(&mut mint0, &mut mint1);
                //what final price is ?
                price = 1.0 / price;
                println!("  Tokens swapped!");         
}

这里的比较是什么方法？ 
    用的是 account publickey 的compare trait方法
     when you see mint0 > mint1, it's actually comparing the raw 32-byte arrays that represent the public keys,