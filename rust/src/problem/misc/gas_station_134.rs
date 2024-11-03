pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // gas contains amout of gas we can add to our tank
    // cost contains amount of fuel to travel to the next station.
    // That implies we only can move rightward one station by one.
    // And we should find an optimal starting point to be able to visit each station.
    // Once we've reached the i station, we can fill up our tank with gas[i] fuel 
    // and move to the next (i + 1) station spedning cost[i] fuel.

    let test_pos = |start: usize| -> Result<usize, usize> {
        let mut i = start;
        let mut fuel = 0;

        loop {
            fuel += gas[i] - cost[i];

            if fuel < 0 {
                return Err(i);
            }

            i = (i + 1) % gas.len();

            if i == start {
                return Ok(start);
            }
        }
    };

    let mut i = 0;

    while i < gas.len() {
        match test_pos(i) {
            Ok(j) => {
                return j as i32;
            }
            Err(j) => {
                if j < i {
                    // if we've wrapped around, then there is no way to walk the entire circle
                    // This is a crucial optimization that leads us to a linear time complexity
                    return -1;
                }

                // Otherwise we can safely skip all stations in between i and j
                // because if we couldn't reach j from i, we won't be able to reach j from any station between i and j
                i = j + 1;
            }
        }
    }

    -1
}

pub fn can_complete_circuit_kadane(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let total_fuel = gas.iter().sum::<i32>();
    let total_cost = cost.iter().sum::<i32>();

    if total_fuel < total_cost {
        return -1;
    }

    let mut start = 0;
    let mut current_fuel = 0;

    for i in 0..gas.len() {
        current_fuel += gas[i] - cost[i];

        if current_fuel < 0 {
            current_fuel = 0;
            start += 1;
        }
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
        assert_eq!(can_complete_circuit_kadane(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);

        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
        assert_eq!(can_complete_circuit_kadane(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}