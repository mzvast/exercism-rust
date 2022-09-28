use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

// 每个桶的数据结构
#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Data(u8, u8); // 1容量，2容量，step

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,        // 1的初始容量
    capacity_2: u8,        // 2的初始容量
    goal: u8,              // 目标容量
    start_bucket: &Bucket, // 从哪个桶开始
) -> Option<BucketStats> {
    // unimplemented!(
    //     "Given one bucket of capacity {}, another of capacity {}, starting with {:?}, find pours to reach {}, or None if impossible",
    //     capacity_1,
    //     capacity_2,
    //     start_bucket,
    //     goal,
    // );

    let get_next = |cur: Data, way: u8| -> Option<Data> {
        match way {
            // 清空1
            0 => {
                if cur.0 > 0 {
                    return Some(Data(0, cur.1));
                }
                None
            }
            1 => {
                // 清空2
                if cur.1 > 0 {
                    return Some(Data(cur.0, 0));
                }
                None
            }
            2 => {
                // 填满1
                if cur.0 < capacity_1 {
                    return Some(Data(capacity_1, cur.1));
                }
                None
            }
            3 => {
                // 填满2
                if cur.1 < capacity_2 {
                    return Some(Data(cur.0, capacity_2));
                }
                None
            }
            4 => {
                // 1倒入2
                if cur.0 > 0 {
                    let max_out = (capacity_2 - cur.1).min(cur.0);
                    return Some(Data(cur.0 - max_out, cur.1 + max_out));
                }
                None
            }
            5 => {
                if cur.1 > 0 {
                    let max_out = (capacity_1 - cur.0).min(cur.1);
                    return Some(Data(cur.0 + max_out, cur.1 - max_out));
                }
                None
            }
            _ => return None,
        }
    };

    // BFS
    let mut q = VecDeque::<Data>::new(); // 当前的状态 (桶1,桶2)

    let mut visited = HashSet::<Data>::new(); // 已经出现过的情况

    if start_bucket == &Bucket::One {
        q.push_back(Data(capacity_1, 0));
        visited.insert(Data(capacity_1, 0));

        // 反向起点阻止
        visited.insert(Data(0, capacity_2));
    } else {
        q.push_back(Data(0, capacity_2));
        visited.insert(Data(0, capacity_2));
        // 反向起点阻止
        visited.insert(Data(capacity_1, 0));
    }

    let mut step = 1; // 总用的步数

    while !q.is_empty() {
        // println!("{:?}", q);

        for _ in 0..q.len() {
            let cur = q.pop_front().unwrap(); // 出队
            // println!("【step:{step}】cur {:?} ", cur);

            // 是否满足条件
            if cur.0 == goal {
                return Some(BucketStats {
                    moves: step,
                    goal_bucket: Bucket::One,
                    other_bucket: cur.1,
                });
            } else if cur.1 == goal {
                return Some(BucketStats {
                    moves: step,
                    goal_bucket: Bucket::Two,
                    other_bucket: cur.0,
                });
            }

            for way in 0..6 {
                if let Some(next) = get_next(cur, way) {
                    if !visited.contains(&next) {
                        // println!("next {:?}, way{way}", next);
                        q.push_back(next);
                        visited.insert(next);
                    }
                }
            }
        }

        step += 1;
    }

    None
}
