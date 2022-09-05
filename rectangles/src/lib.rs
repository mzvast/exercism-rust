pub fn count(lines: &[&str]) -> u32 {
    // unimplemented!("\nDetermine the count of rectangles in the ASCII diagram represented by the following lines:\n{:#?}\n.", lines);

    // 把字符串转成code存到2D数组
    let grid = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();
    // println!("{:?}", grid);

    // 从上到下，从左往右遍历，把找到的点放进points
    // 遍历的过程中用当前节点和之前的节点尝试构成矩形
    let mut points: Vec<(usize, usize)> = Vec::new(); // 已经访问过的节点

    let mut cnt = 0; // 矩形的个数

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != b'+' {
                continue;
            }
            cnt += points
                .iter()
                .filter(|(x0, y0)| is_rect(*x0, *y0, i, j, &grid))
                .count();
            points.push((i, j));
        }
    }

    // println!("{:?}", grid);
    // println!("{:?}", points);

    fn is_rect(x0: usize, y0: usize, x1: usize, y1: usize, grid: &Vec<&[u8]>) -> bool {
        // unimplemented!("根据给定的两个点判定是否可以构成矩形");
        /* *
         * 几何关系如下
         * P0(x0,y0) ------  +Q0(x0,y1)
         *  |                   |
         * +Q1(x1,y0) ------ P1(x1,y1)
         *
         * 点的关系
         * 1、P1在P0的右下方
         * 2、存在Q0和Q1点，且所在位置是+
         * 线的关系
         * 1、（横向）P0和Q0之间，Q1和P1之间，只能是—或者+
         * 2、（纵向）P0和Q1之间，Q0和P1之间，只能是|或者+
         * */

        let ans = x1 > x0
            && y1 > y0
            && grid[x0][y1] == b'+'
            && grid[x1][y0] == b'+'
            && is_horizontal_ok(x0, y0, x1, y1, &grid)
            && is_vertical_ok(x0, y0, x1, y1, &grid);

        // println!("{:?},{:?}", ((x0, y0), (x1, y1)), ans);
        // println!(
        //     "{:?} {:?} {:?} {:?}",
        //     grid[x0][y1] == b'+',
        //     grid[x1][y0] == b'+',
        //     is_horizontal_ok(x0, y0, x1, y1, &grid),
        //     is_vertical_ok(x0, y0, x1, y1, &grid)
        // );

        ans
    }

    fn is_horizontal_ok(x0: usize, y0: usize, x1: usize, y1: usize, grid: &Vec<&[u8]>) -> bool {
        // unimplemented!("判断横向是否满足条件");
        for y in y0 + 1..y1 {
            if (grid[x0][y] != b'-' && grid[x0][y] != b'+')
                || (grid[x1][y] != b'-' && grid[x1][y] != b'+')
            {
                return false;
            }
        }
        true
    }
    fn is_vertical_ok(x0: usize, y0: usize, x1: usize, y1: usize, grid: &Vec<&[u8]>) -> bool {
        // unimplemented!("判断纵向是否满足条件")
        for x in x0 + 1..x1 {
            if (grid[x][y0] != b'|' && grid[x][y0] != b'+')
                || (grid[x][y1] != b'|' && grid[x][y1] != b'+')
            {
                return false;
            }
        }
        true
    }

    cnt as u32
}

// fn main() {
//     let lines = &["  +-+", "    |", "+-+-+", "| | -", "+-+-+"];
//     println!("{}", count(lines));
// }
