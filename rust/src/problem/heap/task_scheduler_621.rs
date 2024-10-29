use std::collections::{BinaryHeap, HashMap};

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut schedule = vec![]; // [A, –, B, –, A, ...]
    let mut queue = BinaryHeap::<(usize, char)>::new(); // (count, task_name)

    tasks.iter().fold(HashMap::<char, usize>::new(), |mut map, &task_name| {
        *map.entry(task_name).or_insert(0) += 1;
        map
    })
        .iter()
        .for_each(|(&task_name, &count)| {
            queue.push((count, task_name));
        });

    while !queue.is_empty() {
        let mut processed = 0;
        let mut stash = vec![];

        // processing chunks of size n + 1: [_, _, _];
        while processed <= n {
            if let Some((mut count, task_name)) = queue.pop() { // we have a task
                schedule.push(task_name);
                count -= 1;

                if count > 0 {
                    stash.push((count, task_name));
                }
            } else if !stash.is_empty() { // we have no tasks
                schedule.push('–'); // idle
            }

            processed += 1;
        }

        stash.iter().for_each(|&task| {
            queue.push(task);
        });
    }

    schedule.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 50), 104);
    }
}