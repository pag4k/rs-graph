use std::result::Result;

mod vertex;
use vertex::*;
mod edge;
use edge::*;

struct Graph<T> {
    vertices: Vec<Vertex<T>>,
    edges: Vec<Edge>,
}

//ADD GET EVERYWHERE FOR THE CASE WHERE THE INDEX DOES NOT EXIST?

impl<T> Graph<T> {

    fn end_vertices(&self, edge_index:usize) -> (usize, usize) {
        (self.edges[edge_index].head, self.edges[edge_index].tail)
    }

    fn opposite(&self, vertex_index:usize, edge_index:usize) -> Result<usize, String> {
        let (head, tail) = self.end_vertices(edge_index);
        if vertex_index == head { Ok(tail) }
        else if vertex_index == tail { Ok(head) }
        else { Err(format!("Edge {} is not incident on vertex {}", edge_index, vertex_index)) }
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

    fn insert_vertex(&mut self, element:T) -> usize {
        let next_index = self.vertices.len();
        self.vertices.push(Vertex {
            index: next_index,
            element: element,
            incident_edges: vec![]});
        next_index
    }

    fn insert_edge(&mut self, head_index:usize, tail_index:usize) -> Result<usize, String> {
        let next_index = self.edges.len();
        if self.vertices.get(head_index).is_none() || self.vertices.get(tail_index).is_none() {
            Err(format!("Cound not insert edge since either node {} or node {} does not exist.", head_index, tail_index))
        }
        self.edges.push(Edge {
            index: next_index,
            tail: tail_index,
            head: head_index,
        });
        Ok(next_index)
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
