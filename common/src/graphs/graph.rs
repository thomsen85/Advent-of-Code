use super::node_translator::NodeTranslator;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WeightedEdge {
    pub to: usize,
    pub weight: i32,
}

impl WeightedEdge {
    pub fn new(to: usize, weight: i32) -> Self {
        Self { to, weight }
    }
}

#[derive(Debug)]
pub struct NamedNodesWeightedGraph<N> {
    pub translation_map: NodeTranslator<String>,
    pub edges: Vec<Vec<WeightedEdge>>,
    pub nodes: Vec<Option<N>>,
}

impl<N> NamedNodesWeightedGraph<N> {
    pub fn new() -> Self {
        Self {
            translation_map: NodeTranslator::new(),
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_or_override_node(&mut self, node_name: String, node_data: N) {
        let node_id = self.translation_map.get_or_insert(node_name);

        let needed_space = ((node_id as i32 + 1) - self.nodes.len() as i32).max(0) as usize;
        for _ in 0..needed_space {
            self.nodes.push(None);
        }

        self.nodes[node_id] = Some(node_data);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32, bi_directional: bool) {
        let max = from.max(to);

        let needed_space = ((max as i32 + 1) - self.edges.len() as i32).max(0) as usize;
        for _ in 0..needed_space {
            self.edges.push(Vec::new());
        }

        self.edges[from].push(WeightedEdge::new(to, weight));

        if bi_directional {
            self.edges[to].push(WeightedEdge::new(from, weight));
        }
    }

    pub fn add_edge_by_name(
        &mut self,
        from: String,
        to: String,
        weight: i32,
        bi_directional: bool,
    ) {
        let from_node = self.translation_map.get_or_insert(from);
        let to_node = self.translation_map.get_or_insert(to);
        self.add_edge(from_node, to_node, weight, bi_directional);
    }

    /// Removes the node and creates a connection between them.
    /// Will not add conneciton if a better one is present
    /// Will replace if a existing edge is heavier
    pub fn truncate_node(&mut self, node: &String) {
        let node_i = self.translation_map.get(node).unwrap();

        for from in &self.edges[node_i].clone() {
            for to in &self.edges[node_i].clone() {
                if from == to {
                    continue;
                }

                if self.edges[from.to]
                    .iter()
                    .find(|edge| edge.to == to.to && edge.weight <= from.weight + to.weight)
                    .is_some()
                {
                    continue;
                }

                self.add_edge(from.to, to.to, from.weight + to.weight, false);

                // Remove both ways
                if let Some(a) = self.edges[from.to]
                    .iter()
                    .position(|edge| edge.to == node_i)
                {
                    self.edges[from.to].swap_remove(a);
                }
                if let Some(a) = self.edges[to.to].iter().position(|edge| edge.to == node_i) {
                    self.edges[to.to].swap_remove(a);
                }
            }
        }
    }
}

impl<N> Default for NamedNodesWeightedGraph<N> {
    fn default() -> Self {
        Self::new()
    }
}
