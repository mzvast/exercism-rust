pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
    let grid = minefield
        .iter()
        .map(|line| line.as_bytes().to_owned())
        .collect::<Vec<_>>();

    // let m = grid.len();
    // let n = grid[0].len();
    let mut ans = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'*' {
                continue;
            }
            let cnt = get_mine_cnt(i as i8, j as i8, &grid);
            ans[i][j] = if cnt > 0 {
                cnt.to_string().as_bytes()[0]
            } else {
                b' '
            }; //u8::try_from(cnt).unwrap();
            // println!("cnt {cnt}");
            // println!("ans[i][j] {i} {j} {:?}", cnt.to_string()); //ans[i][j]);
        }
    }

    // println!("grid {:?}", grid);
    // println!("minefield {:?}", minefield);
    // println!("ans {:?}", ans);

    return ans
        .iter()
        .map(|byte| String::from_utf8(byte.to_vec()).unwrap())
        .collect();

    fn get_mine_cnt(i: i8, j: i8, grid: &Vec<Vec<u8>>) -> usize {
        // 因为我们要处理越界的情况，所以i和j必须是有符号数，不能用usize
        // unimplemented!("计算一个格子周围8个相邻格子中地雷的总数")
        let mut cnt = 0;
        /* *
         *
         * (i-1,j-1)            (i-1,j+1)
         *              (i,j)
         *  (i+1,j-1)           (i+1,j+1)
         *
         */
        // println!("i is {:?}", i);
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                if x == i && y == j {
                    continue;
                }
                let mut res = false;
                if is_mine(x, y, &grid) {
                    res = true;
                    cnt += 1;
                }
                // println!("get_mine_cnt {x} {y} is mine?{:?}", res);
            }
        }
        cnt
    }

    fn is_mine(i: i8, j: i8, grid: &Vec<Vec<u8>>) -> bool {
        let m = grid.len() as i8;
        let n = grid[0].len() as i8;
        // unimplemented
        let mut ans = false;
        if i < 0 || i >= m || j < 0 || j >= n {
            // false
        } else if grid[i as usize][j as usize] == b'*' {
            // true
            ans = true;
        } else {
            // false
        }
        // println!(
        //     "{i} {j} is_mine: {}. grid content is {}",
        //     ans,
        //     String::from_utf8(vec![grid[i as usize][j as usize]]).unwrap()
        // );
        ans
    }
}
