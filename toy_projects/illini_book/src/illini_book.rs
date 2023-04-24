use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct IlliniBook {
    graph: HashMap<i32, Vec<(i32, String)>>,
}

impl IlliniBook {
    pub fn new(people_fpath: &str, relations_fpath: &str) -> Result<Self, Box<dyn Error>> {
        let mut graph = HashMap::new();

        let people_file = File::open(people_fpath)?;
        let people_reader = BufReader::new(people_file);

        for line in people_reader.lines() {
            let uin: i32 = line?.parse()?;
            graph.insert(uin, Vec::new());
        }

        let relations_file = File::open(relations_fpath)?;
        let relations_reader = BufReader::new(relations_file);

        for line in relations_reader.lines() {
            let line = line?;
            let vec: Vec<&str> = line.split(',').collect();
            let uin1: i32 = vec[0].parse()?;
            let uin2: i32 = vec[1].parse()?;
            let relationship = vec[2].to_string();

            graph
                .entry(uin1)
                .or_default()
                .push((uin2, relationship.clone()));

            graph.entry(uin2).or_default().push((uin1, relationship));
        }

        Ok(IlliniBook { graph })
    }

    pub fn are_related(&self, uin_1: i32, uin_2: i32) -> bool {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(uin_1);
        visited.insert(uin_1);

        while let Some(curr_node) = q.pop_front() {
            if curr_node == uin_2 {
                return true;
            }

            if let Some(neighbors) = self.graph.get(&curr_node) {
                for (neighbor, _relation) in neighbors {
                    if !visited.contains(neighbor) {
                        q.push_back(*neighbor);
                        visited.insert(*neighbor);
                    }
                }
            }
        }

        false
    }

    pub fn are_related_relation(&self, uin_1: i32, uin_2: i32, relationship: &str) -> bool {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(uin_1);
        visited.insert(uin_1);

        while let Some(curr_node) = q.pop_front() {
            if curr_node == uin_2 {
                return true;
            }

            if let Some(neighbors) = self.graph.get(&curr_node) {
                for (neighbor, relation) in neighbors {
                    if relation == relationship && !visited.contains(neighbor) {
                        q.push_back(*neighbor);
                        visited.insert(*neighbor);
                    }
                }
            }
        }

        false
    }

    pub fn get_related(&self, uin_1: i32, uin_2: i32) -> i32 {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((uin_1, 0));
        visited.insert(uin_1);

        while let Some((curr_node, length)) = q.pop_front() {
            if curr_node == uin_2 {
                return length;
            }

            if let Some(neighbors) = self.graph.get(&curr_node) {
                for (neighbor, _relation) in neighbors {
                    if !visited.contains(neighbor) {
                        q.push_back((*neighbor, length + 1));
                        visited.insert(*neighbor);
                    }
                }
            }
        }

        -1
    }

    pub fn get_related_relation(&self, uin_1: i32, uin_2: i32, relationship: &str) -> i32 {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((uin_1, 0));
        visited.insert(uin_1);

        while let Some((curr_node, length)) = q.pop_front() {
            if curr_node == uin_2 {
                return length;
            }

            if let Some(neighbors) = self.graph.get(&curr_node) {
                for (neighbor, relation) in neighbors {
                    if relation == relationship && !visited.contains(neighbor) {
                        q.push_back((*neighbor, length + 1));
                        visited.insert(*neighbor);
                    }
                }
            }
        }

        -1
    }

    pub fn get_steps(&self, uin: i32, n: i32) -> Vec<i32> {
        let mut res = Vec::new();

        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((uin, 0));
        visited.insert(uin);

        while let Some((curr_node, length)) = q.pop_front() {
            if length == n {
                res.push(curr_node);
                continue;
            }

            if let Some(neighbors) = self.graph.get(&curr_node) {
                for (neighbor, _relation) in neighbors {
                    if !visited.contains(neighbor) {
                        q.push_back((*neighbor, length + 1));
                        visited.insert(*neighbor);
                    }
                }
            }
        }

        res
    }

    pub fn count_groups(&self) -> u32 {
        let mut count = 0u32;
        let mut visitied = HashSet::new();

        for (node, _edges) in &self.graph {
            if visitied.contains(node) {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(node);
            visitied.insert(node);
            count += 1;

            while let Some(curr_node) = q.pop_front() {
                if let Some(neighbors) = self.graph.get(&curr_node) {
                    for (neighbor, _relation) in neighbors {
                        if !visitied.contains(neighbor) {
                            q.push_back(neighbor);
                            visitied.insert(neighbor);
                        }
                    }
                }
            }
        }

        count
    }

    pub fn count_groups_relation(&self, relationship: &str) -> u32 {
        let mut count = 0u32;
        let mut visitied = HashSet::new();

        for (node, _edges) in &self.graph {
            if visitied.contains(node) {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(node);
            visitied.insert(node);
            count += 1;

            while let Some(curr_node) = q.pop_front() {
                if let Some(neighbors) = self.graph.get(&curr_node) {
                    for (neighbor, relation) in neighbors {
                        if relation == relationship && !visitied.contains(neighbor) {
                            q.push_back(neighbor);
                            visitied.insert(neighbor);
                        }
                    }
                }
            }
        }

        count
    }

    pub fn count_groups_relations(&self, relationships: Vec<&str>) -> u32 {
        let mut count = 0u32;
        let mut visitied = HashSet::new();

        for (node, _edges) in &self.graph {
            if visitied.contains(node) {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(node);
            visitied.insert(node);
            count += 1;

            while let Some(curr_node) = q.pop_front() {
                if let Some(neighbors) = self.graph.get(&curr_node) {
                    for (neighbor, relation) in neighbors {
                        if relationships.contains(&relation.as_str())
                            && !visitied.contains(neighbor)
                        {
                            q.push_back(neighbor);
                            visitied.insert(neighbor);
                        }
                    }
                }
            }
        }

        count
    }

    pub fn print_graph(&self) {
        for (node, edges) in &self.graph {
            for edge in edges {
                println!("Node {}: {} -> {}", node, edge.0, edge.1);
            }
        }
    }
}
