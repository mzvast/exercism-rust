pub struct PascalsTriangle(u32);

// 1
// 1 1
// 1 2 1
// 1 3 3 1
// 1 4 6 4 1
// # ... etc
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // unimplemented!("create Pascal's triangle with {} rows", row_count);
        Self(row_count)
    }
    // 0,0
    // 1,0 1,1
    // 2,0 2,1 2,2
    pub fn rows(&self) -> Vec<Vec<u32>> {
        // unimplemented!();
        let mut ans: Vec<Vec<u32>> = vec![];
        if self.0 > 0 {
            ans.push(vec![1]);
        }
        if self.0 > 1 {
            for i in 1..self.0 {
                let mut buf: Vec<u32> = vec![];
                for j in 0..=i {
                    // println!("i:j->{i}:{j}");
                    // 本质上是ans[i - 1][j-1]+ans[i-1][j]
                    let mut cur = 0;
                    if j >= 1 {
                        cur += ans[(i - 1) as usize][(j - 1) as usize];
                    }
                    if j < i {
                        cur += ans[(i - 1) as usize][j as usize];
                    }
                    buf.push(cur);
                }
                ans.push(buf);
            }
        }
        ans
    }
}
