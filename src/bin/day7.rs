use std::collections::{HashMap, HashSet};

fn main() {
    let lines = include_str!("../../input/d7.txt").lines();
    let mut graphc: HashMap<String, Vec<String>> = HashMap::new();
    let mut graphp: HashMap<String, Vec<(i64, String)>> = HashMap::new();
    lines.for_each(|line| {
        let line: Vec<&str> = line.split(" bags contain ").collect();
        let parent = String::from(line[0].split(' ').collect::<Vec<&str>>().join("_"));
        let childs: Vec<&str> = line[1].split(", ").collect();
        if let None = graphp.get(&parent) {
            graphp.insert(parent.clone(), Vec::new());
        }
        for child in childs.iter() {
            let parts: Vec<&str> = child.split(' ').collect();
            let count = parts[0];
            let name = format!("{}_{}", parts[1], parts[2]);
            if count == "no" {
                continue;
            }
            let count = count.parse::<i64>().unwrap();
            if graphc.get(&name).is_none() {
                graphc.insert(name.clone(), Vec::new());
            }
            graphc.get_mut(&name).unwrap().push(parent.clone());
            graphp.get_mut(&parent).unwrap().push((count, name.clone()));
        }
    });
    let mut potencial_parents: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec!["shiny_gold"];
    while stack.len() != 0 {
        let child = stack.pop().unwrap();
        if let Some(parents) = graphc.get(child) {
            for parent in parents {
                stack.push(parent);
                potencial_parents.insert(&parent);
            }
        }
    }
    println!("{}", potencial_parents.len());

    let mut stack: Vec<(i64, &str)> = vec![(1, "shiny_gold")];
    let mut result = -1;
    while stack.len() != 0 {
        let parent = stack.pop().unwrap();
        result += parent.0;
        if let Some(children) = graphp.get(parent.1) {
            for child in children {
                stack.push((parent.0 * child.0, &child.1));
            }
        }
    }
    println!("{}", result);
}
