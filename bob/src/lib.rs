pub fn reply(message: &str) -> &str {
    // unimplemented!("have Bob reply to the incoming message: {}", message)

    match message.trim() {
        x if x.len() == 0 => "Fine. Be that way!",
        x if x.ends_with("?") && is_yelling(x) => "Calm down, I know what I'm doing!",
        x if x.ends_with("?") => "Sure.",
        x if is_yelling(x) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_yelling(msg: &str) -> bool {
    let has_letters = msg.bytes().filter(|x| x.is_ascii_alphabetic()).count() > 0;

    has_letters && (msg.to_uppercase() == msg)
}
