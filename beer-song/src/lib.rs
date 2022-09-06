pub fn verse(n: u32) -> String {
    if n > 0 {
        format!("{n} bottle{0} of beer on the wall, {n} bottle{0} of beer.\nTake {1} down and pass it around, {2} bottle{3} of beer on the wall.\n",
        get_has_s(n), // 0
        get_it_or_one(n), // 1
        if n-1>0 {(n-1).to_string()}else{"no more".to_owned()} // 2
        ,get_has_s(n-1)) // 3
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned()
    }
}

fn get_it_or_one(n: u32) -> String {
    match n {
        1 => "it".to_owned(),
        _ => "one".to_owned(),
    }
}

fn get_has_s(n: u32) -> String {
    match n {
        1 => "".to_owned(),
        _ => "s".to_owned(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut ans = Vec::new();
    for i in (end..=start).rev() {
        ans.push(verse(i));
    }
    ans.join("\n")
}

// fn main() {
//     let s = sing(3, 0);
//     println!("{:?}", s);
// }
