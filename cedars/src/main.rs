use std::{usize, collections::HashMap};


#[derive(Debug)]
struct Graph {
    vertices: Vec<usize>,
    parents: HashMap<usize, Vec<usize>>,
    children: HashMap<usize, Vec<usize>>,
    neighbors: HashMap<usize, Vec<usize>>,
}


impl Graph {
    fn new(vertices: Vec<usize>) -> Self {
        let mut graph = Graph {
            vertices,
            parents: HashMap::new(),
            children: HashMap::new(),
            neighbors: HashMap::new(),
        };

        for vertex in &graph.vertices {
            graph.parents.insert(*vertex, Vec::new());
            graph.children.insert(*vertex, Vec::new());
            graph.neighbors.insert(*vertex, Vec::new());
        }
        
        return graph;
    }

    fn add_parent(&mut self, vertex: usize, parent: usize) {
        self.parents.entry(vertex).or_insert(Vec::new()).push(parent);
        self.children.entry(parent).or_insert(Vec::new()).push(vertex);
    }

    fn add_child(&mut self, vertex: usize, child: usize) {
        self.children.entry(vertex).or_insert(Vec::new()).push(child);
        self.parents.entry(child).or_insert(Vec::new()).push(vertex);
    }
    
    fn add_neighbor(&mut self, vertex: usize, neighbor: usize) {
        self.neighbors.entry(vertex).or_insert(Vec::new()).push(neighbor);
        self.neighbors.entry(neighbor).or_insert(Vec::new()).push(vertex);
    }
}


fn main() {
    let vertices: Vec<usize> = (0..4).collect();
    let mut graph = Graph::new(vertices);

    graph.add_parent(2, 0);
    graph.add_parent(2, 1);
    graph.add_child(3, 2);
    graph.add_neighbor(0, 1);

    println!("{:?}", graph);
}
