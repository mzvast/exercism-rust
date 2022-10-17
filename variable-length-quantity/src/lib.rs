#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

// 另一种看待这个问题的方法是用 base-128 表示值，然后将除最后一个 base-128 数字之外的所有数字的 MSB 设置为 1。
// https://en.wikipedia.org/wiki/Variable-length_quantity
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    // unimplemented!("Convert the values {:?} to a list of bytes", values)
    let mut ans = Vec::<u8>::new();

    // 每7个一组分组(转成u8，自然丢弃高位。每次右移7位，循环)
    // 最低位MSB为0，其他都为1

    values.iter().rev().for_each(|&num| {
        ans.push(num as u8 & 0b0111_1111); // 最低位，MSB=0
        let mut n = num >> 7;
        while n > 0 {
            ans.push(n as u8 | 0b1000_0000); // MSB=1
            n >>= 7;
        }
    });
    ans.reverse();
    ans
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    // unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
    let mut ans: Vec<u32> = vec![];
    let mut buffer: u32 = 0;

    for (idx, cur) in bytes.iter().enumerate() {
        if buffer.leading_zeros() < 7 {
            // 每次都会向左移动7位，如果先导0不足7个，则必然有一个1，导致溢出
            return Err(Error::Overflow);
        }
        buffer <<= 7;
        buffer |= (cur & 0b0111_1111) as u32;

        let msb = cur & 0b1000_0000;
        if msb == 0 {
            ans.push(buffer);
            buffer = 0;
        } else if idx == bytes.len() - 1 {
            // 数据不完整
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(ans)
}
