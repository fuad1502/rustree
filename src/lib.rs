use std::collections::HashMap;

pub struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
    directed: bool,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
            directed: true,
        }
    }

    pub fn add_vertex(&mut self, id: i32) {
        if !self.adjacency_list.contains_key(&id) {
            self.adjacency_list.insert(id, vec![]);
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) -> Result<(), &str> {
        if self.adjacency_list.contains_key(&from) {
            if self.adjacency_list.contains_key(&to) {
                let list = self.adjacency_list.get_mut(&from).unwrap();
                list.push(to);
                if self.directed {
                    let list = self.adjacency_list.get_mut(&to).unwrap();
                    list.push(from);
                }
                return Ok(());
            }
            return Err("\"to\" vertex doesn't exist in the graph");
        }
        return Err("\"from\" vertex doesn't exist in the graph");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge_success() {
        let mut g = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);
        assert!(g.add_edge(1, 2).is_ok());
    }

    #[test]
    fn add_edge_fail() {
        let mut g = Graph::new();
        assert_eq!(
            g.add_edge(1, 2).err().unwrap(),
            "\"from\" vertex doesn't exist in the graph"
        );
        g.add_vertex(1);
        assert_eq!(
            g.add_edge(1, 2).err().unwrap(),
            "\"to\" vertex doesn't exist in the graph"
        );
    }
}
