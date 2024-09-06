#[cfg(test)]
mod tests;

use super::{Dag, DagEdge, DagNodeId};

pub struct DagValidator<'a> {
    dag: &'a Dag,
}

impl<'a> DagValidator<'a> {
    pub fn new(dag: &'a Dag) -> Self {
        Self { dag }
    }

    pub fn is_valid(&self) -> bool {
        !self.has_cycle() && self.all_edges_reference_existing_nodes()
    }

    pub fn has_cycle(&self) -> bool {
        let adjacency_list = self.build_adjacency_list();
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for node in &self.dag.nodes {
            if self.dfs_has_cycle(&node.id, &adjacency_list, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }

    pub fn all_edges_reference_existing_nodes(&self) -> bool {
        let node_ids: Vec<&DagNodeId> = self.dag.nodes.iter().map(|node| &node.id).collect();

        self.dag
            .edges
            .iter()
            .all(|edge| node_ids.contains(&&edge.from) && node_ids.contains(&&edge.to))
    }

    pub fn would_cause_cycle(&self, new_edge: &DagEdge) -> bool {
        // Create an adjacency list including the new edge.
        let mut adjacency_list = self.build_adjacency_list();

        // Add the new edge to the adjacency list.
        if let Some((_, adjacent_nodes)) = adjacency_list
            .iter_mut()
            .find(|(node_id, _)| *node_id == &new_edge.from)
        {
            adjacent_nodes.push(&new_edge.to);
        } else {
            adjacency_list.push((&new_edge.from, vec![&new_edge.to]));
        }

        // Use the cycle detection function on the modified adjacency list.
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for node in &self.dag.nodes {
            if self.dfs_has_cycle(&node.id, &adjacency_list, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }

    fn build_adjacency_list(&self) -> Vec<(&DagNodeId, Vec<&DagNodeId>)> {
        let mut adjacency_list: Vec<(&DagNodeId, Vec<&DagNodeId>)> = Vec::new();

        for edge in &self.dag.edges {
            if let Some((_, adjacent_nodes)) = adjacency_list
                .iter_mut()
                .find(|(node_id, _)| *node_id == &edge.from)
            {
                adjacent_nodes.push(&edge.to);
            } else {
                adjacency_list.push((&edge.from, vec![&edge.to]));
            }
        }

        adjacency_list
    }

    fn dfs_has_cycle(
        &self,
        node: &DagNodeId,
        adjacency_list: &Vec<(&DagNodeId, Vec<&DagNodeId>)>,
        visited: &mut Vec<DagNodeId>,
        rec_stack: &mut Vec<DagNodeId>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        visited.push(node.clone());
        rec_stack.push(node.clone());

        if let Some((_, adjacent_nodes)) = adjacency_list.iter().find(|(n, _)| *n == node) {
            for &adj_node in adjacent_nodes {
                if self.dfs_has_cycle(adj_node, adjacency_list, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.retain(|n| n != node);
        false
    }
}
