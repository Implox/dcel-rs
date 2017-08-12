//! Current hypothetical API fleshout out part way by Ferris, design thoughts
//! are in `doc/usage_thoughts_v0.1.md`

use ::geometry;


/// Type wrapping of a usize
pub struct FaceId (usize);

/// Type wrapping of a usize
pub struct EdgeId (usize);

/// Type wrapping of a usize
pub struct PointId (usize);

/// Holds IDs of edges, in their order in the graph
pub struct Face {
    edges: Vec<EdgeId>
}

/// Holds the ID of: the point it originated from, it's twin edge, the next
/// edge, and the previous edge.
pub struct Edge {
    source_point: PointId,
    twin: EdgeId,
    next: EdgeId,
    previous: EdgeId
}


/// holds arena for faces, edges, and points plus has methods for building
/// graphs
pub struct DCEL {
    face_arena: Vec<Face>,
    edge_arena: Vec<Edge>,
    point_arena: Vec<geometry::Point2>
}

impl DCEL {
    
    /// creates new DCEL
    pub fn new() -> DCEL {
        DCEL {
            face_arena: vec![],
            edge_arena: vec![],
            point_arena: vec![],
        }
    }

    /// should return total live faces, currently returns total allocated
    pub fn num_faces(&self) -> usize {
        self.face_arena.len()
    }

    /// should return total live edges, currently returns total allocated
    pub fn num_edges(&self) -> usize {
        self.edge_arena.len()
    }

    /// adds face from a list of points and returns the index for the
    /// new face
    pub fn add_face<P, PS>(&mut self, coordinates: PS) -> usize
        where P: Into<geometry::Point2>, PS: IntoIterator<Item=P>
    {
        for coordinate in coordinates.into_iter() {
            // TODO: not implemented!!!
            let _ : geometry::Point2 = coordinate.into();
        }

        // TODO: not implemented!!!
        let f = Face { edges: vec![] };
        self.face_arena.push(f);
        // return index of new face
        self.face_arena.len() - 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const FACE1: [(u32,u32); 3] = [(0,0), (1,1), (1,0)]; // a right triangle
    const FACE2: [(u32,u32); 3] = [(0,0), (1,1), (0,1)]; // another right triangle
    const FACE3: [(u32,u32); 3] = [(0,1), (0,2), (1,1)]; // a third right triangle

    mod add_faces {
        use super::*;

        #[test]
        fn has_correct_face_count() {
            let mut d = DCEL::new();
            let f1_id = d.add_face(&FACE1);
            let f2_id = d.add_face(&FACE2);

            assert_eq!(2, d.num_faces());
        }

        #[test]
        fn has_correct_edge_count() {
            let mut d = DCEL::new();
            let f1_id = d.add_face(&FACE1);
            let f2_id = d.add_face(&FACE2);

            assert_eq!(5, d.num_edges());
        }

    }
}