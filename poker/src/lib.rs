/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
// Card concepts
// rank 数值 2 3 4 5 6 7 8 9 10 J Q K A case中不考虑A当1的情况，暂不处理
//                             11 12 13 14
// suit 花色 S H C D
use std::collections::{HashMap, HashSet};

// 排序权重
// hand_rank https://en.wikipedia.org/wiki/List_of_poker_hands
enum HandRank {
    // 因为不存在大小王，所以FiveOfAKind不考虑
    StraightFlush = 8, // 同花顺
    FourOfAKind = 7,   // 四条
    FullHouse = 6,     // 葫芦
    Flush = 5,         // 同花
    Straight = 4,      // 顺子
    ThreeOfAKind = 3,  // 三条
    TwoPair = 2,       // 两对
    OnePair = 1,       // 一对
    HighCard = 0,      // 高牌
}

// 排序依据
// [main_rank, second_rank,...]
fn get_hand_rank(hand: &str) -> Vec<i32> {
    // unimplemented!("计算每个牌的权重")
    let mut ranks = get_cards_ranks(hand);

    // println!("{:?}", ranks);

    use HandRank::*;
    if is_straight(&mut ranks) && is_flush(hand) {
        // 同花顺
        [StraightFlush as i32, *ranks.iter().max().unwrap(), 0 as i32].to_vec()
    } else if let Some(four) = get_kind_from_cnt(4, &ranks) {
        // 四条
        [
            FourOfAKind as i32,
            four,
            get_kind_from_cnt(1, &ranks).unwrap(),
        ]
        .to_vec()
    } else if let (Some(three), Some(two)) =
        (get_kind_from_cnt(3, &ranks), get_kind_from_cnt(2, &ranks))
    {
        // 葫芦
        [FullHouse as i32, three, two].to_vec()
    } else if is_flush(hand) {
        // 同花
        let mut x = [Flush as i32].to_vec();
        x.append(&mut ranks);
        x
    } else if is_straight(&mut ranks) {
        // 顺子
        [Straight as i32, *ranks.iter().max().unwrap()].to_vec()
    } else if let Some(three) = get_kind_from_cnt(3, &ranks) {
        // 三条
        let mut x = [ThreeOfAKind as i32, three].to_vec();
        x.append(&mut ranks);
        x
    } else if let Some((max, min)) = get_two_pairs(&ranks) {
        // 两对
        let mut x = [TwoPair as i32, max, min].to_vec();
        x.append(&mut ranks);
        x
    } else if let Some(two) = get_kind_from_cnt(2, &ranks) {
        // 一对
        let mut x = [OnePair as i32, two].to_vec();
        x.append(&mut ranks);
        x
    } else {
        let mut x = [HighCard as i32].to_vec();
        x.append(&mut ranks);
        x
    }
}

fn get_two_pairs(ranks: &Vec<i32>) -> Option<(i32, i32)> {
    // unimplemented!("返回两对，max，min")
    let mut pairs = Vec::new();
    let mut cnt = HashMap::new();
    for rank in ranks {
        *cnt.entry(*rank).or_insert(0) += 1;
    }
    for (rank, c) in cnt {
        if c == 2 {
            pairs.push(rank);
        }
    }
    if pairs.len() == 2 {
        Some((*pairs.iter().max().unwrap(), *pairs.iter().min().unwrap()))
    } else {
        None
    }
}

fn get_kind_from_cnt(cnt: i32, ranks: &Vec<i32>) -> Option<i32> {
    // unimplemented!("返回数量为cnt的卡牌rank")
    for x in ranks {
        if ranks.iter().filter(|y| **y == *x).count() as i32 == cnt {
            return Some(*x);
        }
    }
    None
}

fn is_straight(ranks: &mut Vec<i32>) -> bool {
    // unimplemented!("是否是顺子")
    let mut set = HashSet::new();
    ranks.iter().for_each(|x| {
        set.insert(x);
    });
    if set.len() != 5 {
        false
    } else {
        if ranks == &[14, 5, 4, 3, 2] {
            // A 2 3 4 5的情况
            // 把 14删除，在末尾插入 1
            ranks.remove(0);
            ranks.push(1);
            return true;
        }
        let max = set.iter().max().unwrap();
        let min = set.iter().min().unwrap();
        if *max - *min == 4 {
            true
        } else {
            false
        }
    }
}

fn is_flush(hand: &str) -> bool {
    let arr = hand
        .split_whitespace()
        .map(|x| &x[x.len() - 1..])
        .collect::<Vec<&str>>();

    for i in 0..arr.len() - 1 {
        if arr[i] != arr[i + 1] {
            return false;
        }
    }
    true
}

fn get_cards_ranks(hand: &str) -> Vec<i32> {
    // unimplemented!("将排面转成数值，排序后返回")
    let mut ans = hand
        .split_whitespace()
        .map(|x| match &x[..x.len() - 1] {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            n => n.parse().unwrap(),
        })
        .collect::<Vec<i32>>();
    ans.sort_by(|a, b| b.cmp(a));
    ans
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Hand<'a> {
    data: &'a str,       // 原始值
    hand_rank: Vec<i32>, // 手牌的权重
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        Hand {
            data: hand,
            hand_rank: get_hand_rank(hand),
        }
    }
}

pub fn winning_hands<'a>(hands: &'a [&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {:?}, which hand wins?", hands)
    // ["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"]
    // |<- one hand  ->| |<- another hand ->|
    // get_all_max(hands)
    let mut all_hands = Vec::new();
    for &hand in hands {
        let x = Hand::new(hand);
        all_hands.push(x)
    }

    let mut max_val = all_hands[0].clone();
    let mut ans: Vec<Hand> = Vec::new();

    for x in all_hands.iter() {
        if ans.is_empty() || x.hand_rank > max_val.hand_rank {
            ans.clear();
            ans.push(x.clone());
            max_val = x.clone();
        } else if x.hand_rank == max_val.hand_rank {
            ans.push(x.clone());
        }
    }

    // println!("all_hands {:#?}", all_hands);
    // println!("ans {:#?}", ans);
    ans.iter().map(|hand| hand.data).collect()
}

// fn main() {
//     let hands = ["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H", "4S AH AS 8C AD"];
//     let result = winning_hands(&hands);
//     println!("{:?}", result);

// 测试同花顺
// println!("{:?}", is_flush("3H 6H 7H 8H 5H"));
// println!("{:?}", is_flush("3H 6S 7H 8H 5H"));
// }
