// min heap

pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut heap: Vec<i32> = vec![];
    let mut result: Vec<i32> = vec![];

    for item in nums.iter() {
        heap.push(*item);
        maintain_up(&mut heap);
    }

    while let Some(min) = pop(&mut heap) {
        result.push(min);
    }

    result
}

fn maintain_up(heap: &mut [i32]) {
    if heap.is_empty() {
        return;
    }

    let mut index = heap.len() - 1;

    while let Some(parent_idx) = get_parent(heap, index) {
        if heap[index] < heap[parent_idx] {
            heap.swap(index, parent_idx);
            index = parent_idx;
        } else {
            break;
        }
    }
}

fn get_parent(heap: &[i32], index: usize) -> Option<usize> {
    let _ = heap.get(index.checked_sub(1)? / 2)?;

    Some((index - 1) / 2)
}

fn get_children(heap: &[i32], i: usize) -> Option<(Option<usize>, Option<usize>)> {
    let l = heap.get((i * 2) + 1).map(|_| (i * 2) + 1);
    let r = heap.get((i * 2) + 2).map(|_| (i * 2) + 2);

    Some((l, r))
}

fn pop(heap: &mut Vec<i32>) -> Option<i32> {
    if heap.is_empty() {
        return None;
    }

    let a = 0;
    let b = heap.len() - 1;
    heap.swap(a, b);

    let last = heap.pop();

    if heap.len() < 2 {
        return last;
    }

    let mut i = 0;
    while let Some((left, right)) = get_children(heap, i) {
        let idx = match (left, right) {
            (None, None) => break,
            (Some(left_idx), None) => left_idx,
            (None, Some(right_idx)) => right_idx,
            (Some(left_idx), Some(right_idx)) => {
                let condition = heap[left_idx] < heap[right_idx];

                if condition { left_idx } else { right_idx }
            }
        };

        if heap[i] > heap[idx] {
            heap.swap(i, idx);
            i = idx;
        } else {
            break;
        }
    }

    last
}

// println!("{:?}", sort_array(vec![5, 2, 3, 1]));
// println!("{:?}", sort_array(vec![5, 1, 1, 2, 0, 0]));
