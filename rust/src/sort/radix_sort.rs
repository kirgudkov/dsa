pub fn radix_sort(input: &mut [i32]) {
    let mut max = i32::MIN;

    for item in input.iter() {
        max = max.max(*item);
    }

    let mut position = 1;
    while max / position > 0 {
        counting_sort(input, position);
        position *= 10;
    }
}

pub fn counting_sort(input: &mut [i32], position: i32) {
    let mut counts = [0; 10];

    for item in input.iter() {
        // item: 153, position: 10 -> 153 / 10 = 15; 15 % 10 = 5;
        counts[((*item / position) % 10) as usize] += 1;
    }

    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    let mut sorted = vec![0; input.len()];

    for item in input.iter().rev() {
        let idx = ((*item / position) % 10) as usize;
        sorted[counts[idx] - 1] = *item;
        counts[idx] -= 1;
    }

    input.copy_from_slice(&sorted);
}

#[cfg(test)]
mod tests {
    use crate::sort::radix_sort::radix_sort;

    #[test]
    fn test_radix_sort() {
        let mut input = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut input);
        assert_eq!(input, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
