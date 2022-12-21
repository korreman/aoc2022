use fxhash::FxHashMap;
use std::ops::{Index, IndexMut};

/// A graph with immutable structure but mutable node values.
#[rustfmt::skip] pub trait Graph<T> where Self: GraphImpl<T, Map<T> = Self> {}

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

impl<N, X> FromIterator<(N, X)> for VecGraph<X>
where
    N: Iterator<Item = usize>,
{
    fn from_iter<T: IntoIterator<Item = (N, X)>>(iter: T) -> Self {
        Self {
            data: iter
                .into_iter()
                .map(|(ns, v)| VecGraphEntry {
                    value: v,
                    neighbors: ns.collect(),
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
        let data = value
            .data
            .into_iter()
            .map(|(_, entry)| VecGraphEntry {
                value: entry.value,
                neighbors: entry.neighbors.iter().map(|h| indices[h]).collect(),
            })
            .collect();
        VecGraph { data }
    }
}

struct HashGraphEntry<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    value: T,
    neighbors: Vec<H>,
}

impl<H, N, X> FromIterator<(H, N, X)> for HashGraph<H, X>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
    N: Iterator<Item = H>,
{
    fn from_iter<T: IntoIterator<Item = (H, N, X)>>(iter: T) -> Self {
        let data: FxHashMap<H, HashGraphEntry<H, X>> = iter
            .into_iter()
            .map(|(h, n, x)| {
                let entry = HashGraphEntry {
                    value: x,
                    neighbors: n.collect(),
                };
                (h, entry)
            })
            .collect();
        Self { data }
    }
}

/// A graph backed by a hashmap.
pub struct HashGraph<H, T>
where
    H: Copy + PartialEq + Eq + std::hash::Hash,
{
    data: FxHashMap<H, HashGraphEntry<H, T>>,
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
    type Handle = H;

    type Neighbors = std::vec::IntoIter<H>;
    fn neighbors(&self, handle: H) -> Self::Neighbors {
        self.data[&handle].neighbors.clone().into_iter()
    }

    type Map<U> = HashGraph<H, U>;
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
        HashGraph { data }
    }
}
