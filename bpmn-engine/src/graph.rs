use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct Graph<Vid, V = (), E = ()> {
    vertices: HashMap<Vid, V>,
    adjacency: HashMap<Vid, Vec<(Vid, E)>>,
}

impl<Vid, V, E> Graph<Vid, V, E>
where
    Vid: Eq + Hash + Display,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::default(),
            adjacency: HashMap::default(),
        }
    }

    pub fn push_vertex(&mut self, vid: Vid, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(&mut self, from: Vid, to: Vid, edge: E) {
        let adjacent_from = self.adjacency.entry(from).or_default();
        adjacent_from.push((to, edge))
    }
    pub fn has_vertex(&self, vid: &Vid) -> bool {
        self.vertices.contains_key(vid)
    }

    pub fn get_vertex(&self, vid: &Vid) -> Option<&V> {
        self.vertices.get(vid)
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = (&Vid, &V)> {
        self.vertices.iter()
    }

    pub fn get_edge(&self, from: Vid, to: Vid) -> Option<&E> {
        self.adjacency.get(&from).and_then(|edges| {
            edges
                .iter()
                .find(|(curr_to_to, edge)| *curr_to_to == to)
                .map(|(_, edge)| edge)
        })
    }

    pub fn adjacent(self: &Self, vid: &Vid) -> Vec<&Vid> {
        if let Some(value) = self.adjacency.get(vid) {
            value.iter().map(|(vid, _e)| vid).collect()
        } else {
            vec![]
        }
    }
}
