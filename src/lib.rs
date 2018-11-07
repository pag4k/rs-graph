use std::result::Result;

mod vertex;
use vertex::*;
mod edge;
use edge::*;

pub struct Graph<T> {
    vertices: Vec<Vertex<T>>,
    edges: Vec<Edge>,
}

//ADD GET EVERYWHERE FOR THE CASE WHERE THE INDEX DOES NOT EXIST?

impl<T> Graph<T> {

    fn new() -> Self {
        Graph {
            vertices: vec!(),
            edges: vec!(),
        }
    }

    fn end_vertices(&self, edge_index:usize) -> (usize, usize) {
        (self.edges[edge_index].head, self.edges[edge_index].tail)
    }

    fn opposite(&self, vertex_index:usize, edge_index:usize) -> Result<usize, String> {
        let (head, tail) = self.end_vertices(edge_index);
        if vertex_index == head { Ok(tail) }
        else if vertex_index == tail { Ok(head) }
        else { Err(format!("Edge {} is not incident on vertex {}.", edge_index, vertex_index)) }
    }

    fn are_adjacent(&self, vertex_index1:usize, vertex_index2:usize) -> bool {
        let (smallest_incident_eges_list_index, other_index) =
            if self.vertices[vertex_index1].incident_edges.len() <= self.vertices[vertex_index2].incident_edges.len()
                { (vertex_index1, vertex_index2) }
            else
                { (vertex_index2, vertex_index1) };
        self.vertices[smallest_incident_eges_list_index].incident_edges
            .iter()
            .any(|&incident_edge| self.opposite(vertex_index1, incident_edge) == Ok(other_index))
    }

    fn replace(&mut self, vertex_index:usize, element:T) {
        //TODO: CLONE THE ELEMENT?
        self.vertices[vertex_index].element = element;
    }

    pub fn insert_vertex(&mut self, element:T) -> usize {
        let next_index = self.vertices.len();
        self.vertices.push(Vertex {
            index: next_index,
            element: element,
            incident_edges: vec![]
        });
        next_index
    }

    pub fn insert_edge(&mut self, head_index:usize, tail_index:usize) -> Result<usize, String> {
        let next_index = self.edges.len();
        if self.vertices.get(head_index).is_none() || self.vertices.get(tail_index).is_none() {
            //FIXME: FOR SOME REASON, IT WON'T LET ME REMOVE THE RETURN.
            return Err(format!("Cound not insert edge since either node {} or node {} does not exist.", head_index, tail_index));
        }
        self.edges.push(Edge {
            index: next_index,
            tail: tail_index,
            head: head_index,
        });
        self.vertices[head_index].incident_edges.push(next_index);
        self.vertices[tail_index].incident_edges.push(next_index);
        Ok(next_index)
    }

    //fn remove_vertex(&mut self, vertex_index:usize) -> T {}

    //fn remove_edge(&mut self, edge_index:usize) {}

    fn vertices(&self) -> &Vec<Vertex<T>> {
        &self.vertices
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn incident_edges(&self, vertex_index:usize) -> &Vec<usize> {
        &self.vertices[vertex_index].incident_edges
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test1() {
        let mut graph: Graph<usize> = Graph::new();
        let vertex0 = graph.insert_vertex(5);
        let vertex1 = graph.insert_vertex(10);
        let vertex2 = graph.insert_vertex(15);
        let vertex3 = graph.insert_vertex(9);
        let edge0 = graph.insert_edge(vertex0, vertex1).unwrap();
        let edge1 = graph.insert_edge(vertex0, vertex2).unwrap();
        let edge2 = graph.insert_edge(vertex1, vertex3).unwrap();
        assert!(graph.end_vertices(edge0) == (vertex0, vertex1));
        assert!(graph.end_vertices(edge1) != (vertex0, vertex1));
        assert!(graph.opposite(vertex3, edge2) == Ok(vertex1));
        assert!(graph.opposite(vertex3, edge0) == Err("Edge 0 is not incident on vertex 3.".to_string()));
        assert!(graph.are_adjacent(vertex0, vertex1));
        assert!(graph.are_adjacent(vertex1, vertex0));
        assert!(!graph.are_adjacent(vertex0, vertex0));
        assert!(!graph.are_adjacent(vertex0, vertex3));




    }
}
