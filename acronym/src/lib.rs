pub fn abbreviate(phrase: &str) -> String {
    // unimplemented!("Given the phrase '{phrase}', return its acronym");

    let ans = phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'') // 断句
        .filter(|s| !s.is_empty())
        // 拍平
        .flat_map(|word: &str| {
            // 取首字母
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_ascii_uppercase()) // 从第一个小写开始
                    .filter(|c| c.is_ascii_uppercase()), // 取所有大写字母
            )
        })
        .collect::<String>()
        .to_uppercase();

    ans
}

// fn main() {
//     // println!("{:?}", abbreviate("Halley's Comet"));
//     // println!("{:?}", abbreviate("HyperText Markup Language"));
//     println!("{:?}", abbreviate("PHP: Hypertext Preprocessor"));
// }
