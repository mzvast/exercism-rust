pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // unimplemented!(
    //     "Using the binary search algorithm, find the element '{}' in the array '{:?}' and return its index.",
    //     key,
    //     array
    // );

    if array.len() == 0 {
        return None;
    }

    let (mut head, mut tail) = (0, array.len() - 1);

    while tail - head > 3 {
        let mid = (head + tail) / 2;
        if array[mid] < key {
            head = mid + 1;
        } else {
            tail = mid;
        }
    }

    for i in head..=tail {
        if array[i] == key {
            return Some(i);
        }
    }

    None
}
