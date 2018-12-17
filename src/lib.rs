use std::result::Result;

mod vertex;
use vertex::Vertex;
mod edge;
use edge::Edge;

pub struct Graph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
}

impl<V, E> Default for Graph<V, E> {
    fn default() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
        }
    }
}

impl<V, E> Graph<V, E> {
    pub fn end_vertices(&self, edge_index: usize) -> Option<(usize, usize)> {
        match self.edges.get(edge_index) {
            Some(edge) => Some((edge.head, edge.tail)),
            None => None,
        }
    }

    pub fn opposite(&self, vertex_index: usize, edge_index: usize) -> Option<usize> {
        let (head, tail) = match self.end_vertices(edge_index) {
            Some((a, b)) => (a, b),
            None => return None,
        };
        if vertex_index == head {
            Some(tail)
        } else if vertex_index == tail {
            Some(head)
        } else {
            None
        }
    }

    fn are_adjacent(&self, vertex_index1: usize, vertex_index2: usize) -> bool {
        let (smallest_incident_eges_list_index, other_index) = match (
            self.vertices.get(vertex_index1),
            self.vertices.get(vertex_index2),
        ) {
            (Some(a), Some(b)) => {
                if a.incident_edges.len() <= b.incident_edges.len() {
                    (vertex_index1, vertex_index2)
                } else {
                    (vertex_index2, vertex_index1)
                }
            }
            _ => return false,
        };
        self.vertices[smallest_incident_eges_list_index]
            .incident_edges
            .iter()
            .any(|&incident_edge| self.opposite(vertex_index1, incident_edge) == Some(other_index))
    }

    fn replace(&mut self, vertex_index: usize, element: V) {
        self.vertices[vertex_index].element = element;
    }

    pub fn insert_vertex(&mut self, element: V) -> usize {
        let next_index = self.vertices.len();
        self.vertices.push(Vertex {
            index: next_index,
            element: element,
            incident_edges: vec![],
        });
        next_index
    }

    pub fn insert_edge(
        &mut self,
        element: E,
        head_index: usize,
        tail_index: usize,
    ) -> Result<usize, String> {
        let next_index = self.edges.len();
        if self.vertices.get(head_index).is_none() || self.vertices.get(tail_index).is_none() {
            return Err(format!(
                "Cound not insert edge since either node {} or node {} does not exist.",
                head_index, tail_index
            ));
        }
        self.edges.push(Edge {
            index: next_index,
            element: element,
            tail: tail_index,
            head: head_index,
        });
        self.vertices[head_index].incident_edges.push(next_index);
        self.vertices[tail_index].incident_edges.push(next_index);
        Ok(next_index)
    }

    //fn remove_vertex(&mut self, vertex_index:usize) -> T {}

    //fn remove_edge(&mut self, edge_index:usize) {}

    pub fn get_vertex_element(&self, vertex_index: usize) -> Option<&V> {
        match self.vertices.get(vertex_index) {
            Some(vertex) => Some(&vertex.element),
            None => None,
        }
    }

    pub fn get_edge_element(&self, edge_index: usize) -> Option<&E> {
        match self.edges.get(edge_index) {
            Some(edge) => Some(&edge.element),
            None => None,
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex<V>> {
        &self.vertices
    }

    pub fn edges(&self) -> &Vec<Edge<E>> {
        &self.edges
    }

    pub fn incident_edges(&self, vertex_index: usize) -> Option<&Vec<usize>> {
        match self.vertices.get(vertex_index) {
            Some(vertex) => Some(&vertex.incident_edges),
            None => None,
        }
    }

    pub fn outgoing_edges(&self, vertex_index: usize) -> Option<Vec<usize>> {
        match self.vertices.get(vertex_index) {
            Some(vertex) => Some(
                vertex
                    .incident_edges
                    .iter()
                    .filter(|edge_index| self.edges[**edge_index].tail == vertex_index)
                    .cloned()
                    .collect(),
            ),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let mut graph: Graph<usize, usize> = Graph::default();
        let vertex0 = graph.insert_vertex(5);
        let vertex1 = graph.insert_vertex(10);
        let vertex2 = graph.insert_vertex(15);
        let vertex3 = graph.insert_vertex(9);
        let edge0 = graph.insert_edge(1, vertex0, vertex1).unwrap();
        let edge1 = graph.insert_edge(1, vertex0, vertex2).unwrap();
        let edge2 = graph.insert_edge(1, vertex1, vertex3).unwrap();
        assert!(graph.end_vertices(edge0) == Some((vertex0, vertex1)));
        assert!(graph.end_vertices(edge1) != Some((vertex0, vertex1)));
        assert!(graph.opposite(vertex3, edge2) == Some(vertex1));
        assert!(graph.opposite(vertex3, edge0) == None);
        assert!(graph.are_adjacent(vertex0, vertex1));
        assert!(graph.are_adjacent(vertex1, vertex0));
        assert!(!graph.are_adjacent(vertex0, vertex0));
        assert!(!graph.are_adjacent(vertex0, vertex3));
    }
}
