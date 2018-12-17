pub struct Vertex<T> {
    pub index: usize,
    pub element: T,
    pub incident_edges: Vec<usize>,
}
