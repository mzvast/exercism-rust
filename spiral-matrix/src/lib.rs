pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // unimplemented!(
    //     "Function that returns the spiral matrix of square size {}",
    //     size
    // );

    // 旋转矩阵
    // 1 2 3
    // 8 9 4
    // 7 6 5

    let mut x = 0; // row
    let mut y = -1; // col
    let mut dir = 0; // 方向
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut cnt = 0;

    let mut ans: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];

    while cnt < size * size {
        let (dx, dy) = dirs[dir];
        x += dx;
        y += dy;
        if x < size as i32
            && x >= 0
            && y < size as i32
            && y >= 0
            && ans[x as usize][y as usize] == 0
        {
            ans[x as usize][y as usize] = cnt + 1;
            cnt += 1;
        } else {
            // 回退，转方向
            x -= dx;
            y -= dy;
            dir = (dir + 1) % 4;
        }
    }

    ans
}
