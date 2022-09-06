pub fn verse(n: u32) -> String {
    if n > 0 {
        format!("{n} bottle{} of beer on the wall, {n} bottle{} of beer.\nTake {} down and pass it around, {} bottle{} of beer on the wall.\n",
        get_has_s(n),
        get_has_s(n),
        get_it_or_one(n),
        if n-1>0 {(n-1).to_string()}else{"no more".to_owned()}
        ,get_has_s(n-1))
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned()
    }
}

fn get_it_or_one(n: u32) -> String {
    if n <= 1 {
        "it".to_owned()
    } else {
        "one".to_owned()
    }
}

fn get_has_s(n: u32) -> String {
    if n == 1 {
        "".to_owned()
    } else {
        "s".to_owned()
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
