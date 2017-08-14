use ::geometry::*;
use ::arena::*;

// ArenaID types for a DCEL
arena_id!(VertexId);
arena_id!(HalfEdgeId);
arena_id!(FaceId);

#[derive(Debug, PartialEq)]
pub struct Vertex {
    coord: Point2,
    outgoing_edge: Option<HalfEdgeId>,

    deleted: bool,
}

make_deleteable!(Vertex);
impl Vertex {
    pub fn new(coord: Point2) -> Vertex {
        Vertex { coord: coord, outgoing_edge: None, deleted: false }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HalfEdge {
    origin: VertexId,
    twin: Option<HalfEdgeId>,
    next: Option<HalfEdgeId>,
    prev: Option<HalfEdgeId>,
    face: FaceId,

    deleted: bool,
}

make_deleteable!(HalfEdge);
impl HalfEdge {
    pub fn new(origin: VertexId, face: FaceId) -> HalfEdge {
        HalfEdge {
            origin: origin,
            twin: None,
            next: None,
            prev: None,
            face: face,

            deleted: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Face {
    incident_edge: Option<HalfEdgeId>,

    deleted: bool
}

make_deleteable!(Face);
impl Face {
    pub fn new() -> Face {
        Face { incident_edge: None, deleted: false }
    }
}

type VertexArena = Arena<Vertex, VertexId>;
type HalfEdgeArena = Arena<HalfEdge, HalfEdgeId>;
type FaceArena = Arena<Face, FaceId>;

#[derive(Debug)]
pub struct DCEL {
    vertices: VertexArena,
    edges: HalfEdgeArena,
    faces: FaceArena,

    outer_face: FaceId,
}

impl DCEL {
    pub fn new() -> DCEL {
        let mut f_arena = FaceArena::new();
        let outer_face = Face::new();
        let outer_face_id = f_arena.add(outer_face);

        DCEL {
            vertices: VertexArena::new(),
            edges: HalfEdgeArena::new(),
            faces: f_arena,

            outer_face: outer_face_id,
        }
    }

    pub fn add_vertex(&mut self, point: Point2) -> VertexId {
        // Create vertex, add it to arena, return id
        let vert = Vertex::new(point);
        let id = self.vertices.add(vert);
        return id;
    }

    pub fn add_half_edge(&mut self, origin_id: VertexId, face_id: FaceId) -> HalfEdgeId {
        // Create edge, add it to the arena
        let edge = HalfEdge::new(origin_id, face_id);
        let edge_id = self.edges.add(edge);
        
        // Update the origin of the edge
        let mut origin = &mut self.vertices[origin_id];
        origin.outgoing_edge = Some(edge_id);

        // Update the face of the edge
        let mut face = &mut self.faces[face_id];
        face.incident_edge = Some(edge_id);

        // Return id
        return edge_id;
    }

    /// Adds a new face into this DCEL.
    pub fn add_face(&mut self) -> FaceId {
        // Create face, add it to arena, return id
        let face = Face::new();
        let id = self.faces.add(face);
        return id;
    }
}