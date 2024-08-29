pub fn radix_sort(input: &mut [i32]) {
    let max = *input.iter().max().unwrap();
    let mut pos = 1usize;

    while max as usize / pos > 0 {
        counting_sort(input, pos);
        pos *= 10;
    }
}

pub fn counting_sort(input: &mut [i32], pos: usize) {
    let mut counts = [0; 10];

    for &item in input.iter() {
        counts[(item as usize / pos) % 10] += 1;
    }

    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    let mut sorted = vec![0; input.len()];

    input.iter().rev().for_each(|&item| {
        let i = (item as usize / pos) % 10;
        sorted[counts[i] - 1] = item;
        counts[i] -= 1;
    });

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
