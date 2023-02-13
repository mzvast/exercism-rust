use regex::Regex;
pub fn translate(input: &str) -> String {
    // unimplemented!(
    //     "Using the Pig Latin text transformation rules, convert the given input '{input}'"
    // );
    let rule1 = Regex::new(r"^([aeiou]|xr|yt)+").unwrap();
    let rule2 = Regex::new(r"^[^aeiou]+").unwrap();
    let rule3 = Regex::new(r".*?qu").unwrap();
    let rule4 = Regex::new(r"^[^aeiou]+?y").unwrap();

    if input.contains(" ") {
        return input
            .split(" ")
            .map(|word| translate(word))
            .collect::<Vec<_>>()
            .join(" ");
    }

    if rule1.is_match(input) {
        return input.to_owned() + "ay";
    } else if let Some(mat) = rule3.find(input) {
        return input.chars().skip(mat.end()).collect::<String>() + mat.as_str() + "ay";
    } else if let Some(mat) = rule4.find(input) {
        return input
            .chars()
            .skip(mat.end() - 1)
            .chain(input.chars().take(mat.end() - 1))
            .collect::<String>()
            + "ay";
    } else if let Some(mat) = rule2.find(input) {
        return input.chars().skip(mat.end()).collect::<String>() + mat.as_str() + "ay";
    }

    " ".to_owned()
}

fn main() {
    // println!("{:?}", translate("xray"));
    // println!("{:?}", translate("square"));
    // println!("{:?}", translate("chair"));
}
