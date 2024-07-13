// O(n^2) easy to understand solution:
pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

    let mut dp = vec![1; envelopes.len()];

    envelopes.iter().enumerate().skip(1).for_each(|(i, c)| {
        envelopes.iter().enumerate().take(i).for_each(|(j, p)| {
            if c[0] > p[0] && c[1] > p[1] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        });
    });

    *dp.iter().max().unwrap()
}

// O(n log n) magic solution:
//
// pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
//     envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
// 
//     let mut dp = vec![];
// 
//     for env in envelopes {
//         let mut l = 0;
//         let mut r = dp.len() as i32;
// 
//         while l < r {
//             let m = l + (r - l) / 2;
// 
//             if dp[m as usize] < env[1] {
//                 l = m + 1;
//             } else {
//                 r = m;
//             }
//         }
// 
//         if l as usize == dp.len() {
//             dp.push(env[1]);
//         } else {
//             dp[l as usize] = env[1];
//         }
//     }
// 
//     dp.len() as i32
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(max_envelopes(envelopes), 3);

        let envelopes = vec![vec![4, 5], vec![6, 7], vec![2, 3]];
        assert_eq!(max_envelopes(envelopes), 3);
    }
}
