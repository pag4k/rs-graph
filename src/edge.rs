pub struct Edge<T> {
    pub index: usize,
    pub element: T,
    pub tail: usize,
    pub head: usize,
}
