// 9 8 7
// 5 3 2
// 6 6 7

use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // unimplemented!(
    //     "find the saddle points of the following matrix: {:?}",
    //     input
    // )

    // 行里面的最大值，是否是列里面的最小值
    // 行的最大值构成一个Set，列的最小值构成一个Set，同时出现在两个Set的即是答案

    let rows = input.len();
    let cols = input[0].len();

    if rows == 0 || cols == 0 {
        return vec![];
    }

    let mut max_row_set = HashSet::<(usize, usize)>::new();

    for i in 0..rows {
        let mut max = input[i][0];
        let mut max_js = vec![0]; // 相等的点的集合

        for j in 1..cols {
            if input[i][j] > max {
                max = input[i][j];
                max_js = vec![j]; // 重置
            } else if input[i][j] == max {
                max_js.push(j); // 追加
            }
        }
        for k in max_js {
            max_row_set.insert((i, k));
        }
    }

    let mut min_col_set = HashSet::<(usize, usize)>::new();

    for j in 0..cols {
        let mut min = input[0][j];
        let mut min_is = vec![0];
        for i in 0..rows {
            if input[i][j] < min {
                min = input[i][j];
                min_is = vec![i];
            } else if input[i][j] == min {
                min_is.push(i);
            }
        }
        for k in min_is {
            min_col_set.insert((k, j));
        }
    }

    max_row_set
        .iter()
        .filter(|p| min_col_set.contains(p))
        .map(|p| *p)
        .collect::<Vec<_>>()
}
