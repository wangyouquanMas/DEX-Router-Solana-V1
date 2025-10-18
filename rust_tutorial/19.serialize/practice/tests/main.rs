use std::io::{self,Write};

#[derive(Debug)]
struct MyStruct{
    id: u32,
    name: String,
}

impl MyStruct{
    //Binary serialization method
    fn try_serialize<W: Write>(&self, writer:&mut W) -> io::Result<()>{
        //Write a fixed-size header (discriminator)
        writer.write_all(&self.id.to_le_bytes())?;

        //Write the name as raw bytes
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u32;
        writer.write_all(&name_len.to_le_bytes())?;
        writer.write_all(name_bytes)?;

        Ok(())
    }
}

#[test]
fn test_try_serialize(){
    let my_struct = MyStruct{
        id: 42,
        name: "Alice".to_string(),
    };

    let mut serialized_data = Vec::new();

    assert!(my_struct.try_serialize(&mut serialized_data).is_ok());

     // Now check if the serialized data is correct
        // The expected data:
        // 1. `id` (u32) -> 42 -> [42, 0, 0, 0] (little-endian)
        // 2. `name_len` (u32) -> 5 (length of "Alice") -> [5, 0, 0, 0]
        // 3. `name` (String) -> "Alice" -> [65, 108, 105, 99, 101]      
    
        
    //打印字节数组时，Rust 会显示字节的 十进制 值。即 [42, 0, 0, 0] 
    // 表示这四个字节的十进制值分别为 42、0、0 和 0。
    println!("Serialized data: {:?}", serialized_data);

    let expected: Vec<u8> = vec![
        42, 0, 0, 0, // `id` (u32)
        5, 0, 0, 0,  // `name_len` (u32)
        65, 108, 105, 99, 101, // "Alice" (bytes)
    ];

    // Check if the serialized data matches the expected data
    assert_eq!(serialized_data, expected);
}