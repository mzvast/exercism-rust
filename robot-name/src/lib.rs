pub struct Robot {
    name: String,
}

static mut uuid: (i32, i32, i32) = (0, 0, 000); // AA000
                                                // RX837
impl Robot {
    pub fn new() -> Self {
        // unimplemented!("Construct a new Robot struct.");
        let name = get_name_from_uuid();
        Self { name }
    }

    pub fn name(&self) -> &str {
        // unimplemented!("Return the reference to the robot's name.");
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        // unimplemented!("Assign a new unique name to the robot.");
        self.name = get_name_from_uuid();
    }
}

fn get_name_from_uuid() -> String {
    let mut cur: (i32, i32, i32) = unsafe { uuid };
    cur.2 += 1;
    if cur.2 == 1000 {
        cur.2 = 0; // 进位
        cur.1 += 1;
    }
    if cur.1 == 26 {
        cur.1 = 0; // 进位
        cur.0 += 1;
    }
    if cur.0 == 26 {
        cur.0 = 0; // 进位
    }

    let mut ans = String::new();

    ans.push((b'A' + cur.0 as u8) as char);
    ans.push((b'A' + cur.1 as u8) as char);
    ans.push_str(cur.2.to_string().as_str());

    unsafe {
        uuid = cur;
    }

    ans
}
