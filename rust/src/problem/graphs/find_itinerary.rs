use std::collections::HashMap;

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();

    tickets.iter().for_each(|ticket| {
        graph.entry(&ticket[0])
            .or_default()
            .push(&ticket[1]);
    });

    graph.values_mut().for_each(|v| {
        v.sort_unstable_by(|a, b| b.cmp(a));
    });

    let mut result = vec![];

    fn dfs(v: &String, graph: &mut HashMap<&String, Vec<&String>>, result: &mut Vec<String>) {
        while let Some(next) = graph.get_mut(v).and_then(|v| v.pop()) {
            dfs(next, graph, result);
        }

        result.insert(0, v.clone());
    }

    dfs(&"JFK".to_string(), &mut graph, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_itinerary() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ];
        assert_eq!(find_itinerary(tickets), vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);

        let tickets = vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        assert_eq!(find_itinerary(tickets), vec!["JFK", "MUC", "LHR", "SFO", "SJC"]);

        let tickets = vec![
            vec!["JFK".to_string(), "KUL".to_string()],
            vec!["JFK".to_string(), "NRT".to_string()],
            vec!["NRT".to_string(), "JFK".to_string()],
        ];
        assert_eq!(find_itinerary(tickets), vec!["JFK", "NRT", "JFK", "KUL"]);
    }
}