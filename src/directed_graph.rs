pub struct Node {
    name: String,
}

impl Node {
    pub fn new(name: String) -> Node {
        return Node { name: name };
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

pub struct DirectedGraph {
    pub nodes: Vec<Node>,
    edges: Vec<Vec<i64>>,
}

impl DirectedGraph {
    pub fn new() -> DirectedGraph {
        return DirectedGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
    }

    pub fn get_node_names(&self) -> Vec<String> {
        return self.nodes.iter().map(|n| n.name.clone()).collect();
    }

    pub fn add_node_if_not_exists(&mut self, name: String) {
        if self.nodes.iter().any(|n| n.name == name) {
            return;
        }

        self.nodes.push(Node::new(name));

        for v in &mut self.edges {
            v.push(0);
        }
        self.edges.push(std::iter::repeat(0).take(self.nodes.len()).collect());
    }

    pub fn add_edge_by_names(&mut self, start: &str, end: &str, weight: i64) {
        let start_index = self.nodes.iter().position(|n| n.name == *start).unwrap();
        let end_index = self.nodes.iter().position(|n| n.name == *end).unwrap();

        self.edges[start_index][end_index] = weight;
    }

    pub fn sum_of_path_weight_products(&self, start: &str) -> i64{
        let current_node = (self.nodes.iter().position(|n| n.name == *start).unwrap(), 0);
        let total_sum = 0;

        return self.sum_of_path_weight_product_traversal(current_node, total_sum);
    }

    fn sum_of_path_weight_product_traversal(&self, current_node: (usize, i64), total_sum: i64) -> i64 {
        let new_edges: Vec<(usize, i64)> = self.edges[current_node.0].iter().enumerate()
            .filter(|e| *e.1 > 0)
            .map(|e| (e.0, *e.1))
            .collect();

        if new_edges.len() == 0 {
            return 0;
        } else {
            let mut new_sum = total_sum;
            for e in new_edges {
                new_sum += e.1 + e.1 * self.sum_of_path_weight_product_traversal(e, total_sum);
            }

            return new_sum;
        }
    }

    pub fn iter_from_node(&self, start: &str) -> DepthFirstIter {
        let start_index = self.nodes.iter().position(|n| n.name == *start).unwrap();

        return DepthFirstIter::new(&self.nodes, &self.edges, start_index);
    }
}

impl std::fmt::Display for DirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Nodes: {}\n", self.nodes.iter().map(|n| n.name.clone()).collect::<Vec<String>>().join(", "))?;

        for v in &self.edges {
            writeln!(f, "  {}", v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "))?;
        }

        return std::fmt::Result::Ok(());
    }
}

pub struct DepthFirstIter<'a> {
    nodes: &'a Vec<Node>,
    edges: &'a Vec<Vec<i64>>,
    visited_indices: std::collections::HashSet<usize>,
    stack: Vec<(usize, i64)>,
}

impl<'a> DepthFirstIter<'a> {
    pub fn new(nodes: &'a Vec<Node>, edges: &'a Vec<Vec<i64>>, start_index: usize) -> DepthFirstIter<'a> {
        return DepthFirstIter {
            nodes: nodes,
            edges: edges,
            visited_indices: std::collections::HashSet::new(),
            stack: vec!((start_index, 0)),
        };
    }
}

impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = (&'a Node, i64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(weighted_e) => {
                self.visited_indices.insert(weighted_e.0);

                let new_edges: Vec<(usize, i64)> = self.edges[weighted_e.0].iter().enumerate()
                    .filter(|e| *e.1 > 0)
                    .filter(|e| !self.visited_indices.contains(&e.0))
                    .map(|e| (e.0, *e.1))
                    .collect();
                for existing_edge in new_edges {
                    self.stack.push(existing_edge);
                }

                return Some((&self.nodes[weighted_e.0], weighted_e.1));
            },
            None => return None,
        };
    }
}
