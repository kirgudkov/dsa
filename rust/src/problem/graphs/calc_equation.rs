use std::collections::{HashMap, HashSet, VecDeque};

// https://leetcode.com/problems/evaluate-division
pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut ds = DisjointSet::new();

    equations.iter().zip(values.iter()).for_each(|(eq, val)| {
        ds.union(&eq[0], &eq[1], *val);
    });

    queries.iter().map(|q| {
        if !ds.map.contains_key(&q[0]) || !ds.map.contains_key(&q[1]) {
            return -1.0;
        }

        let (a_root, a_val) = ds.find(&q[0]);
        let (b_root, b_val) = ds.find(&q[1]);

        if a_root == b_root {
            a_val / b_val
        } else {
            -1.0
        }
    }).collect()
}

struct DisjointSet {
    map: HashMap<String, (String, f64)>,
}

impl DisjointSet {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn find(&mut self, x: &String) -> (String, f64) {
        if !self.map.contains_key(x) {
            self.map.insert(x.clone(), (x.clone(), 1.0));
        }

        let (mut root, mut val) = self.map.get(x)
            .unwrap()
            .clone();

        if root != *x {
            let (new_root, new_val) = self.find(&root);
            self.map.insert(x.clone(), (new_root.clone(), new_val * val));

            root = new_root;
            val *= new_val;
        }

        (root, val)
    }

    fn union(&mut self, a: &String, b: &String, val: f64) {
        let (a_root, a_val) = self.find(a);
        let (b_root, b_val) = self.find(b);

        if a_root != b_root {
            self.map.insert(a_root.clone(), (b_root.clone(), b_val * val / a_val));
        }
    }
}

// Adjacency list + BFS approach. Much easier to understand imo.
pub fn calc_equation_bfs(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph = HashMap::new();

    equations.iter().zip(values.iter()).for_each(|(eq, val)| {
        graph.entry(eq[0].clone()).or_insert(vec![]).push((eq[1].clone(), *val));
        graph.entry(eq[1].clone()).or_insert(vec![]).push((eq[0].clone(), 1.0 / val));
    });

    let bfs = |nom: &str, denom: &str| -> f64 {
        if !graph.contains_key(nom) || !graph.contains_key(denom) {
            return -1.0;
        }

        let mut visited = HashSet::from([nom]);
        let mut q = VecDeque::from([(nom, 1.0)]);

        while let Some((v, val)) = q.pop_front() {
            if v == denom {
                return val;
            }

            for (n, _val) in graph.get(v).unwrap() {
                if !visited.contains(n.as_str()) {
                    q.push_back((n, val * _val));
                    visited.insert(n);
                }
            }
        }

        -1.0
    };

    queries.iter().map(|q| {
        bfs(&q[0], &q[1])
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_equation() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ];
        let res = vec![6.0, 0.5, -1.0, 1.0, -1.0];
        assert_eq!(calc_equation(equations, values, queries), res);

        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
            vec!["cd".to_string(), "bc".to_string()],
        ];
        let res = vec![3.75, 0.4, 5.0, 0.2];
        assert_eq!(calc_equation(equations, values, queries), res);
    }

    #[test]
    fn test_calc_equation_bfs() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ];
        let res = vec![6.0, 0.5, -1.0, 1.0, -1.0];
        assert_eq!(calc_equation_bfs(equations, values, queries), res);

        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
            vec!["cd".to_string(), "bc".to_string()],
        ];
        let res = vec![3.75, 0.4, 5.0, 0.2];
        assert_eq!(calc_equation_bfs(equations, values, queries), res);
    }
}