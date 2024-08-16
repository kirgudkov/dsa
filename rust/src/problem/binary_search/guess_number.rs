fn guess_number(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;

    while l < r {
        let m = l + (r - l) / 2;

        match guess(m) {
            -1 => r = m - 1,
            1 => l = m + 1,
            _ => return m,
        }
    }

    l
}

fn guess(_: i32) -> i32 {
    0
}

