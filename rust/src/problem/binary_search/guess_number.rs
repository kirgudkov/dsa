fn guess_number(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    while left < right {
        let mid = left + (right - left) / 2;

        match guess(mid) {
            -1 => right = mid - 1,
            1 => left = mid + 1,
            _ => return mid,
        }
    }

    left
}

fn guess(_: i32) -> i32 {
    0
}

