#!/usr/bin/env python3

import struct

# 16进制指令数据
hex_data = "8934edd4d7756c6801003c00b80b0000b004000000000000"

# 转换为字节
data = bytes.fromhex(hex_data)

print(f"原始16进制数据: {hex_data}")
print(f"数据长度: {len(data)} 字节")
print(f"字节数据: {data}")

# 分析数据结构
# 前8字节是discriminator (8934edd4d7756c68)
discriminator = data[:8]
print(f"\nDiscriminator: {discriminator.hex()}")

# 剩余数据是参数
params_data = data[8:]
print(f"参数数据: {params_data.hex()}")

# 根据CreateAmmConfig指令的参数结构解析
# 参数顺序: index(u16), tick_spacing(u16), trade_fee_rate(u32), protocol_fee_rate(u32), fund_fee_rate(u32)

if len(params_data) >= 16:  # 2+2+4+4+4 = 16字节
    index = struct.unpack('<H', params_data[0:2])[0]  # u16, little-endian
    tick_spacing = struct.unpack('<H', params_data[2:4])[0]  # u16, little-endian
    trade_fee_rate = struct.unpack('<I', params_data[4:8])[0]  # u32, little-endian
    protocol_fee_rate = struct.unpack('<I', params_data[8:12])[0]  # u32, little-endian
    fund_fee_rate = struct.unpack('<I', params_data[12:16])[0]  # u32, little-endian
    
    print(f"\n解析结果:")
    print(f"index: {index}")
    print(f"tick_spacing: {tick_spacing}")
    print(f"trade_fee_rate: {trade_fee_rate}")
    print(f"protocol_fee_rate: {protocol_fee_rate}")
    print(f"fund_fee_rate: {fund_fee_rate}")
    
    # 验证与输出结果的一致性
    print(f"\n验证:")
    print(f"index (1): {index == 1}")
    print(f"tick_spacing (60): {tick_spacing == 60}")
    print(f"trade_fee_rate (3000): {trade_fee_rate == 3000}")
    print(f"protocol_fee_rate (1200): {protocol_fee_rate == 1200}")
    print(f"fund_fee_rate (0): {fund_fee_rate == 0}")
else:
    print("参数数据长度不足") 