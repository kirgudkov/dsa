struct Solution {
    p_sum: Vec<i32>,
}

impl Solution {
    fn new(weights: Vec<i32>) -> Self {
        let mut p_sum = weights;

        for i in 1..p_sum.len() {
            p_sum[i] += p_sum[i - 1];
        }

        Self { p_sum }
    }

    fn pick_index(&self) -> i32 {
        let last = *self.p_sum.last().unwrap() as u32;
        let target = (rand::random::<u32>() % last + 1) as i32;
        
        let mut l = 0;
        let mut r = self.p_sum.len() as i32 - 1;
        
        while l < r {
            let m = l + (r - l) / 2;
            
            if self.p_sum[m as usize] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        
        l
    }
}
// [1,3] -> [1,4];