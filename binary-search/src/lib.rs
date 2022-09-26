pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // unimplemented!(
    //     "Using the binary search algorithm, find the element '{}' in the array '{:?}' and return its index.",
    //     key,
    //     array
    // );

    if array.is_empty() {
        return None;
    }

    let mut head = 0 as i8;
    let mut tail: i8 = array.len() as i8 - 1;

    while tail - head > 3 {
        let mid = (head + tail) / 2;
        if array[mid as usize] < key {
            head = mid + 1;
        } else {
            tail = mid;
        }
    }

    for i in head..=tail {
        if array[i as usize] == key {
            return Some(i as usize);
        }
    }

    None
}
