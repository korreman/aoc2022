use fxhash::FxHashMap;
use std::ops::{Index, IndexMut, Range};

/// A graph with immutable structure but mutable node values.
#[rustfmt::skip] pub trait Graph<T> where Self: GraphImpl<T, Map<T> = Self> {}

/// Actual implementation of the graph class.
pub trait GraphImpl<T>
where
    Self: Index<Self::Node, Output = T> + IndexMut<Self::Node, Output = T>,
{
    /// A reference to a node in a graph.
    /// This should index the equivalent node between mapped graphs.
    type Node: Copy;

    /// Get the neighbors for `node` in the graph.
    fn neighbors(&self, node: Self::Node) -> Self::Neighbors;
    type Neighbors: Iterator<Item = Self::Node>;

    /// Get handles to all of the nodes in the graph.
    /// No ordering is guaranteed.
    fn nodes(&self) -> Self::AllNodes;
    type AllNodes: Iterator<Item = Self::Node>;

    /// Mapping function, creates a new graph with the exact same structure.
    fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> Self::Map<U>;
    type Map<U>: GraphImpl<U, Node = Self::Node>;
}

/// An index graph backed by a `Vec`.
#[derive(Clone)]
pub struct VecGraph<T> {
    pub start: usize,
    data: Vec<VecGraphEntry<T>>,
}

#[derive(Clone)]
struct VecGraphEntry<T> {
    value: T,
    neighbors: Vec<usize>,
}

impl<T> VecGraph<T> {
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.data[src].neighbors.push(dst);
    }
}

impl<N, X> FromIterator<(N, X)> for VecGraph<X>
where
    N: IntoIterator<Item = usize>,
{
    fn from_iter<T: IntoIterator<Item = (N, X)>>(iter: T) -> Self {
        Self {
            start: 0,
            data: iter
                .into_iter()
                .map(|(n, v)| VecGraphEntry {
                    value: v,
                    neighbors: n.into_iter().collect(),
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
    type Node = usize;

    fn neighbors(&self, node: usize) -> Self::Neighbors {
        self.data[node].neighbors.clone().into_iter()
    }
    type Neighbors = std::vec::IntoIter<usize>;

    fn nodes(&self) -> Self::AllNodes {
        0..self.data.len()
    }
    type AllNodes = Range<usize>;

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
        VecGraph {
            start: self.start,
            data,
        }
    }
    type Map<U> = VecGraph<U>;
}

impl<H, T> From<HashGraph<H, T>> for VecGraph<T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    fn from(value: HashGraph<H, T>) -> Self {
        let indices: FxHashMap<H, usize> = value
            .data
            .iter()
            .enumerate()
            .map(|(idx, (&h, _))| (h, idx))
            .collect();
        let start = indices[&value.start];
        let data = value
            .data.into_values().map(|entry| VecGraphEntry {
                value: entry.value,
                neighbors: entry.neighbors.iter().map(|h| indices[h]).collect(),
            })
            .collect();
        VecGraph { start, data }
    }
}

#[derive(Clone)]
struct HashGraphEntry<H, T>
//where
//    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    value: T,
    neighbors: Vec<H>,
}

/// A graph backed by a hashmap.
#[derive(Clone)]
pub struct HashGraph<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    pub start: H,
    data: FxHashMap<H, HashGraphEntry<H, T>>,
}

impl<H, N, X> FromIterator<(H, N, X)> for HashGraph<H, X>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
    N: Iterator<Item = H>,
{
    fn from_iter<T: IntoIterator<Item = (H, N, X)>>(iter: T) -> Self {
        let mut peekable = iter.into_iter().peekable();
        let start = peekable.peek().expect("empty iterator").0;
        let data: FxHashMap<H, HashGraphEntry<H, X>> = peekable
            .map(|(h, n, x)| {
                let entry = HashGraphEntry {
                    value: x,
                    neighbors: n.collect(),
                };
                (h, entry)
            })
            .collect();
        Self { start, data }
    }
}

impl<H, T> Index<H> for HashGraph<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    type Output = T;
    fn index(&self, index: H) -> &Self::Output {
        &self.data[&index].value
    }
}

impl<H, T> IndexMut<H> for HashGraph<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    fn index_mut(&mut self, index: H) -> &mut Self::Output {
        &mut self.data.get_mut(&index).unwrap().value
    }
}

impl<H, T> Graph<T> for HashGraph<H, T> where H: Copy + PartialEq + Eq + std::hash::Hash {}
impl<H, T> GraphImpl<T> for HashGraph<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    type Node = H;

    fn neighbors(&self, node: H) -> Self::Neighbors {
        self.data[&node].neighbors.clone().into_iter()
    }
    type Neighbors = std::vec::IntoIter<H>;

    fn nodes(&self) -> Self::AllNodes {
        self.data.keys().cloned().collect::<Vec<H>>().into_iter()
    }
    type AllNodes = std::vec::IntoIter<H>;

    fn map<U, F: FnMut(&T) -> U>(&self, mut f: F) -> Self::Map<U> {
        let data = self
            .data
            .iter()
            .map(|(handle, node)| {
                let entry = HashGraphEntry {
                    value: f(&node.value),
                    neighbors: node.neighbors.clone(),
                };
                (*handle, entry)
            })
            .collect();
        HashGraph {
            start: self.start,
            data,
        }
    }
    type Map<U> = HashGraph<H, U>;
}
