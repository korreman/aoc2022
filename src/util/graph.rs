use std::ops::{Index, IndexMut};

/// A graph with immutable structure but mutable node values.
pub trait Graph<T> where Self: GraphImpl<T, Map<T> = Self> {}

/// Actual implementation of the graph class.
pub trait GraphImpl<T>
where
    Self: Index<Self::Handle, Output = T> + IndexMut<Self::Handle, Output = T>,
{
    /// A reference to a node in a graph.
    /// This should index the equivalent node between mapped graphs.
    type Handle: Copy;

    /// An iterator over neighboring nodes.
    type Neighbors: Iterator<Item = Self::Handle>;
    /// Get the neighbors for `handle` in the graph.
    fn neighbors(&self, handle: Self::Handle) -> Self::Neighbors;

    /// Result of mapping, should be the type itself.
    type Map<U>: GraphImpl<U, Handle = Self::Handle>;
    /// Mapping function, creates a new graph with the exact same structure.
    fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> Self::Map<U>;
}

/// An index graph backed by a `Vec`.
pub struct VecGraph<T> {
    data: Vec<VecGraphEntry<T>>,
}

struct VecGraphEntry<T> {
    value: T,
    neighbors: Vec<usize>,
}

impl<T> VecGraph<T> {
    /// Generates a new graph from a list of nodes and their neighbor indices.
    pub fn new(nodes: Vec<(T, Vec<usize>)>) -> Self {
        Self {
            data: nodes
                .into_iter()
                .map(|(v, ns)| VecGraphEntry {
                    value: v,
                    neighbors: ns,
                })
                .collect(),
        }
    }
}

impl<T> Index<usize> for VecGraph<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index].value
    }
}

impl<T> IndexMut<usize> for VecGraph<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index].value
    }
}

impl<T> Graph<T> for VecGraph<T> {}
impl<T> GraphImpl<T> for VecGraph<T> {
    type Handle = usize;

    type Neighbors = std::vec::IntoIter<usize>;
    fn neighbors(&self, handle: usize) -> Self::Neighbors {
        self.data[handle].neighbors.clone().into_iter()
    }

    type Map<U> = VecGraph<U>;
    fn map<U, F: FnMut(&T) -> U>(&self, mut f: F) -> Self::Map<U> {
        let data = self
            .data
            .iter()
            .map(|node| {
                let neighbors = node.neighbors.clone();
                VecGraphEntry {
                    value: f(&node.value),
                    neighbors,
                }
            })
            .collect();
        VecGraph { data }
    }
}
