use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = Vec::new(); // brackets only

    let mut map = HashMap::new();
    map.insert(b')', b'(');
    map.insert(b']', b'[');
    map.insert(b'}', b'{');

    for code in string.bytes() {
        match code {
            b'{' | b'[' | b'(' => {
                s.push(code);
            }
            b'}' | b']' | b')' => {
                if s.pop() != map.get(&code).cloned() { // cloned is to make Option<&u8>->Option<u8>
                    return false;
                }
            }
            _ => {}
        }
    }
    return s.is_empty();
}
