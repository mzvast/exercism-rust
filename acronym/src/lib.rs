pub fn abbreviate(phrase: &str) -> String {
    // unimplemented!("Given the phrase '{phrase}', return its acronym");

    let ans = phrase
        // .split_ascii_whitespace()
        .split(|c: char| !c.is_alphabetic() && c != '\'') // 断句
        .filter(|s| !s.is_empty())
        .map(|word: &str| {
            if word.chars().all(|c| c.is_uppercase()) {
                // 都是大写字母，取第一个
                return word[0..1].to_uppercase();
            }
            // 否则首字母+后面的大写字母
            word[0..1].to_uppercase()
                + word[1..]
                    .chars()
                    .filter(|c| c.is_uppercase())
                    .collect::<String>()
                    .as_str()
        })
        .collect::<String>();

    ans
}

// fn main() {
//     // println!("{:?}", abbreviate("Halley's Comet"));
//     // println!("{:?}", abbreviate("HyperText Markup Language"));
//     println!("{:?}", abbreviate("PHP: Hypertext Preprocessor"));
// }
