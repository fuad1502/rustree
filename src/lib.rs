use std::collections::{HashMap, VecDeque};

pub struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
    directed: bool,
}

pub struct BfsFunc<'a> {
    vertex_processing_early: Box<dyn FnMut(i32) + 'a>,
    vertex_processing_late: Box<dyn FnMut(i32) + 'a>,
    edge_processing_in_tree: Box<dyn FnMut(i32, i32) + 'a>,
    edge_processing_out_tree: Box<dyn FnMut(i32, i32) + 'a>,
}

impl<'a> BfsFunc<'a> {
    pub fn new() -> BfsFunc<'a> {
        BfsFunc {
            vertex_processing_early: Box::new(|_| {}),
            vertex_processing_late: Box::new(|_| {}),
            edge_processing_in_tree: Box::new(|_, _| {}),
            edge_processing_out_tree: Box::new(|_, _| {}),
        }
    }
}

enum BfsState {
    Undiscovered,
    Discovered,
    Processed,
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

    pub fn bfs(&self, root: i32, mut bfs_functions: BfsFunc) -> Result<(), &str> {
        if !self.adjacency_list.contains_key(&root) {
            return Err("Root vertex doesn't exist in the graph");
        }

        let mut states = HashMap::new();
        self.bfs_init(&mut states);

        states.insert(root, BfsState::Discovered);
        let mut for_processing = VecDeque::new();
        for_processing.push_back(root);

        while for_processing.len() != 0 {
            let curr_vertex = for_processing.pop_front().unwrap();
            (bfs_functions.vertex_processing_early)(curr_vertex);
            states.insert(curr_vertex, BfsState::Processed);
            let al = self.adjacency_list.get(&curr_vertex).unwrap();
            for &adj_vertex in al {
                match states.get(&adj_vertex).unwrap() {
                    BfsState::Undiscovered => {
                        for_processing.push_back(adj_vertex);
                        states.insert(adj_vertex, BfsState::Discovered);
                        (bfs_functions.edge_processing_in_tree)(curr_vertex, adj_vertex);
                    }
                    BfsState::Discovered => {
                        (bfs_functions.edge_processing_out_tree)(curr_vertex, adj_vertex);
                    }
                    BfsState::Processed => {}
                }
            }
            (bfs_functions.vertex_processing_late)(curr_vertex);
        }
        Ok(())
    }

    fn bfs_init(&self, states: &mut HashMap<i32, BfsState>) {
        for id in self.adjacency_list.keys() {
            states.insert(*id, BfsState::Undiscovered);
        }
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

    #[test]
    fn bfs_success() {
        let mut g = Graph::new();
        for id in 1..=8 {
            g.add_vertex(id);
        }
        g.add_edge(1, 2).unwrap();
        g.add_edge(1, 7).unwrap();
        g.add_edge(1, 8).unwrap();
        g.add_edge(2, 3).unwrap();
        g.add_edge(2, 5).unwrap();
        g.add_edge(2, 7).unwrap();
        g.add_edge(3, 4).unwrap();
        g.add_edge(3, 5).unwrap();
        g.add_edge(4, 5).unwrap();
        g.add_edge(5, 6).unwrap();

        let mut parents: HashMap<i32, i32> = HashMap::new();
        let mut bfs_functions = BfsFunc::new();
        let edge_processing_in_tree = |from, to| {
            parents.insert(to, from);
        };
        bfs_functions.edge_processing_in_tree = Box::new(edge_processing_in_tree);
        assert!(g.bfs(1, bfs_functions).is_ok());

        let mut expected_parents: HashMap<i32, i32> = HashMap::new();
        expected_parents.insert(2,1);
        expected_parents.insert(7,1);
        expected_parents.insert(8,1);
        expected_parents.insert(3,2);
        expected_parents.insert(5,2);
        expected_parents.insert(4,3);
        expected_parents.insert(6,5);
        assert_eq!(parents, expected_parents);
    }

    #[test]
    fn bfs_fail() {
        let g = Graph::new();
        let bfs_functions = BfsFunc::new();
        assert_eq!(
            g.bfs(1, bfs_functions).err().unwrap(),
            "Root vertex doesn't exist in the graph"
        );
    }
}
