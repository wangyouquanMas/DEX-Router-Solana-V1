目标：
1. 掌握序列化方法 【背景 #[account]宏】
2. 在#[account] macro中的应用
3. Writer的用法
4. to_le_bytes用法
5. as_bytes用法

内容：
1. 序列化方法1：Binary Seralization 
#[account] macro 采用 binary seralization,将 Rust结构体直接转化为
原始字节

举例：
fn try_serialize<W: std::io::Write>(&self,write: &mut W)
其中
Uses std::io::Write trait for binary output
Writes raw bytes to a stream
No human-readable format like JSON or XML


2. 在#[account]中的应用
该实现采用了two-stage approach:
Stage 1: Header/Identifier Writing

if writer.write_all(#disc).is_err() {
    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
}
Writes a fixed-size header (8-byte discriminator)
Acts as a type identifier and validation mechanism

Stage 2: Payload Serialization
if AnchorSerialize::serialize(self, writer).is_err() {
    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
}
Serializes the actual data payload
Uses Anchor's serialization framework


3. Writer用法
writer 通常是一个实现了 std::io::Write trait 的对象，它用于将数据写入某种输出流，比如内存缓冲区、文件、网络连接等
序列化过程中，writer 扮演了 数据写入目的地 的角色。

方法：
1. write(): 将数据写入目标流，返回写入的字节数和任何可能的错误。
2. write_all(): 类似于 write()，但是会一直写，直到所有的数据都被写入（如果出现错误会返回）

举例：
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut output = Vec::new();  // 创建一个可写的内存缓冲区
    output.write_all(b"Hello, world!")?;  // 向内存缓冲区写入数据
    println!("{:?}", output);  // 输出写入的数据
    Ok(())
}
这列Vec实现了Write trait,所以可以直接调用其方法


4. to_le-bytes用法
用于将基本数据类型（如整数类型）转换为字节数组。to_le_bytes 生成一个按 小端字节序（little-endian） 排列的字节数组。小端字节序是指低位字节存储在内存的低地址位置，高位字节存储在内存的高地址位置。
to_le_bytes 适用的类型
这个方法主要用于整数类型（i32, u32, i64, u64 等），将它们转换为字节数组。

举例：
假设我们有一个 u32 类型的值，我们可以使用 to_le_bytes 将其转换为小端字节序的字节数组：
fn main() {
    let number: u32 = 42;  // u32 类型的数字
    let bytes = number.to_le_bytes();  // 将 u32 转换为小端字节数组

    println!("{:?}", bytes);  // 输出: [42, 0, 0, 0]
}
解释：
42（十进制）在二进制中表示为 00000000 00000000 00000000 00101010（32 位）。

to_le_bytes() 将该 u32 转换为小端字节序的字节数组，即 42 存储在最低字节，其他字节是 0：

42 -> 0x2A（十六进制）

小端字节序：[42, 0, 0, 0]

补充：低位字节和高位字节
假设我们有一个 u32 类型的数字 0x12345678（十六进制），它的二进制表示为：
00000001 00110010 01010110 01111000
低位字节是 0x78，它是表示数字的最低有效字节（least significant byte）。
高位字节是 0x12，它是表示数字的最高有效字节（most significant byte）。
小端字节序（Little-endian）表示：
在小端字节序中，内存中的存储顺序会是：[0x78, 0x56, 0x34, 0x12]
即低位字节 0x78 存储在最小的内存地址（内存的开头），高位字节 0x12 存储在内存的高地址。


5. as_bytes用法
as_bytes 是 String 和 &str 类型的方法，用于将字符串转换为字节切片（&[u8]）。它不会进行任何复制或重新分配内存，只是返回一个指向字符串内容的字节切片。返回的字节切片是 UTF-8 编码的原始字节序列。

举例：
fn main() {
    let s = String::from("Hello, world!");
    let bytes = s.as_bytes();  // 将 String 转换为字节切片

    println!("{:?}", bytes);  // 输出: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
}
解释：
"Hello, world!" 这个字符串在 UTF-8 编码中被表示为一系列字节。

as_bytes() 将字符串转换为字节切片 &[u8]，即它返回该字符串的 UTF-8 字节表示：

'H' -> 72 (ASCII 值)

'e' -> 101 (ASCII 值)

'l' -> 108 (ASCII 值)

...

',' -> 44 (ASCII 值)

' ' -> 32 (ASCII 值)

'w' -> 119 (ASCII 值)

...