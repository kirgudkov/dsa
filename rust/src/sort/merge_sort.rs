// Divide and conquer approach: 
// recursively split the array in half, sort each half and merge them back together
pub fn merge_sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        return vec;
    }

    merge(
        merge_sort(vec[0..vec.len() / 2].to_vec()),
        merge_sort(vec[vec.len() / 2..].to_vec()),
    )
}

fn merge(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut result = vec![];

    while i < vec_1.len() && j < vec_2.len() {
        if vec_1[i] < vec_2[j] {
            result.push(vec_1[i]);
            i += 1;
        } else {
            result.push(vec_2[j]);
            j += 1;
        }
    }

    while i < vec_1.len() {
        result.push(vec_1[i]);
        i += 1;
    }

    while j < vec_2.len() {
        result.push(vec_2[j]);
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(vec![4, 3, 2, 1]), vec![1, 2, 3, 4]);
        assert_eq!(merge_sort(vec![4, 3, 2, 1, 5]), vec![1, 2, 3, 4, 5]);
    }
}
