use std::ops::{Index, IndexMut};

/// A graph with immutable structure, but mutable node values.
/// This is an outer trait that allows paramterization over the node data type.
pub trait Graph {
    /// A reference to a node in a graph.
    /// This should index the equivalent node between mapped graphs.
    type Handle: Copy;
    /// The graph type, parameterized over the node data `T`.
    type Graph<T>: GraphInner<Self::Handle, T>;
    /// Mapping function, creates a new graph with the exact same structure.
    fn map<T, U, F: FnMut(&T) -> U>(graph: &Self::Graph<T>, f: F) -> Self::Graph<U>;
}

/// The actual (inner) graph type of a [Graph].
pub trait GraphInner<H, T>
where
    Self: Index<H, Output = T> + IndexMut<H, Output = T>,
{
    /// An iterator over neighboring nodes to some handle.
    type Neighbors: Iterator<Item = H>;
    /// Get the neighbors for handle `H`.
    fn neighbors(&self, handle: H) -> Self::Neighbors;
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

impl<T> GraphInner<usize, T> for VecGraph<T> {
    type Neighbors = std::vec::IntoIter<usize>;

    fn neighbors(&self, handle: usize) -> Self::Neighbors {
        self.data[handle].neighbors.clone().into_iter()
    }
}

impl<X> Graph for VecGraph<X> {
    type Handle = usize;

    type Graph<T> = VecGraph<T>;

    fn map<T, U, F: FnMut(&T) -> U>(graph: &Self::Graph<T>, mut f: F) -> Self::Graph<U> {
        let data = graph
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
