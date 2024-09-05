#[cfg(test)]
mod tests;

// TODO:  Check on: Topological sorting
// TODO: https://en.wikipedia.org/wiki/Topological_sorting

use std::ops::{Add, Deref};

use super::blockchain::wallet::PublicKey;

#[derive(Debug)]
pub enum DagGraphError {
    NonExistentFrom,
    NonExistentTo,
    NoNodes,
    NoEdges,
    CycleRisk,
    Other(String),
}

#[derive(Debug)]
pub struct Dag {
    nodes: Vec<DagNode>,
    edges: Vec<DagEdge>,
    last_node: Option<DagNodeId>,
    last_edge: Option<DagEdgeId>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            last_node: None,
            last_edge: None,
        }
    }
    pub fn add_node<N: Into<DagNodeData>>(&mut self, input: N) -> Result<DagNodeId, String> {
        let new_data: DagNodeData = input.into();

        for item in self.nodes.iter() {
            if item.data == new_data {
                return Err("Trying to add an existing DagNode".into());
            }
        }

        let new_node_id = self
            .last_node
            .as_ref()
            .map(|last_node_id| DagNodeId::new(**last_node_id + 1))
            .unwrap_or(DagNodeId::new(1));

        let new_node = DagNode {
            id: new_node_id.clone(),
            data: new_data,
        };

        self.nodes.push(new_node);
        self.last_node = Some(new_node_id.clone());

        Ok(new_node_id)
    }

    pub fn add_edge<I: AsRef<DagNodeId>>(&mut self, from: I, to: I) -> Result<DagEdgeId, String> {
        let new_from = from.as_ref();
        let new_to = to.as_ref();

        let mut from_found = false;
        let mut to_found = false;

        for item in self.nodes.iter() {
            if &item.id == new_from {
                from_found = true;
            }
            if &item.id == new_to {
                to_found = true;
            }

            if from_found && to_found {
                break;
            }
        }

        if !(from_found) {
            return Err("Trying to add an DagEdge without a valid `from` DagNode".into());
        }

        if !(to_found) {
            return Err("Trying to add an DagEdge without a valid `to` DagNode".into());
        }

        let new_edge_id = self
            .last_edge
            .as_ref()
            .map(|last_edge_id| DagEdgeId::new(**last_edge_id + 1))
            .unwrap_or(DagEdgeId::new(1));

        let new_edge = DagEdge {
            id: new_edge_id.clone(),
            from: new_from.clone(),
            to: new_to.clone(),
        };

        self.edges.push(new_edge);

        self.last_edge = Some(new_edge_id.clone());

        Ok(new_edge_id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DagEdge {
    id: DagEdgeId,
    from: DagNodeId,
    to: DagNodeId,
}

#[derive(Debug, PartialEq)]
pub struct DagNode {
    id: DagNodeId,
    data: DagNodeData,
}
#[derive(Debug, PartialEq)]
pub struct DagNodeData(PublicKey);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DagNodeId(u128);

impl DagNodeId {
    const fn new(new_id: u128) -> Self {
        Self(new_id)
    }
}

impl Deref for DagNodeId {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<u128> for DagNodeId {
    type Output = u128;

    fn add(self, rhs: u128) -> Self::Output {
        self.0.add(rhs)
    }
}

impl AsRef<DagNodeId> for DagNodeId {
    fn as_ref(&self) -> &DagNodeId {
        &self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DagEdgeId(u128);

impl Deref for DagEdgeId {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DagEdgeId {
    const fn new(new_id: u128) -> Self {
        Self(new_id)
    }
}

impl Add<u128> for DagEdgeId {
    type Output = u128;

    fn add(self, rhs: u128) -> Self::Output {
        self.0.add(rhs)
    }
}
