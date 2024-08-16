const MOD: u64 = 1_000_000_007;

pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();

    // Dark magic happens here a.k.a. "Fast exponentiation";
    // Regular pow would overflow for large numbers, so we use this trick to avoid that.
    let pow = |x: u64| -> u64 {
        let mut res = 1u64;
        let mut num = x;
        let mut base = 2;

        while num > 0 {
            if num % 2 == 1 {
                res = (res * base) % MOD;
            }

            base = (base * base) % MOD;
            num /= 2;
        }

        res
    };

    // Alternative implementation of pow function using recursion:
    // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Recursive_version
    // fn pow(x: u64, n: u64) -> u64 {
    //     if n == 0 {
    //         1
    //     } else if n % 2 == 0 {
    //         pow(x * x, n / 2)
    //     } else {
    //         x * pow(x, n - 1)
    //     }
    // }

    let mut count = 0;

    nums.iter()
        .take_while(|&x| *x <= target / 2)
        .enumerate()
        .for_each(|(i, &num)| {
            let mut l = i;
            let mut r = nums.len() - 1;

            while l <= r {
                let m = l + (r - l) / 2;

                if num + nums[m] <= target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }

            count = (count + pow((l - 1 - i) as u64)) % MOD;
        });

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subseq() {
        let result = num_subseq(vec![3, 5, 6, 7], 9);
        assert_eq!(result, 4);

        let result = num_subseq(vec![3, 3, 6, 8], 10);
        assert_eq!(result, 6);

        let result = num_subseq(vec![2, 3, 3, 4, 6, 7], 12);
        assert_eq!(result, 61);

        let result = num_subseq(vec![5, 2, 4, 1, 7, 6, 8], 16);
        assert_eq!(result, 127);
    }
}