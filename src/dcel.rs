use geometry::*;
use arena::*;

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
        Vertex {
            coord: coord,
            outgoing_edge: None,
            deleted: false,
        }
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

    deleted: bool,
}

make_deleteable!(Face);
impl Face {
    pub fn new() -> Face {
        Face {
            incident_edge: None,
            deleted: false,
        }
    }
}

type VertexArena = Arena<Vertex, VertexId>;
type HalfEdgeArena = Arena<HalfEdge, HalfEdgeId>;
type FaceArena = Arena<Face, FaceId>;

#[derive(Debug)]
pub struct DCEL {
    pub vertices: VertexArena,
    pub edges: HalfEdgeArena,
    pub faces: FaceArena,

    pub outer_face: FaceId,
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

    /// Adds a new vertex to this DCEL.
    pub fn add_vertex(&mut self, point: Point2) -> VertexId {
        // Create vertex, add it to arena, return id
        let vert = Vertex::new(point);
        let id = self.vertices.add(vert);
        return id;
    }

    /// Adds a new half-edge to this DCEL.
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

    /// Given a half-edge, computes the cycle of edges it's a part of.
    /// Panicks if a half-edge with no `next` field is encountered.
    pub fn get_cycle_from(&self, start: HalfEdgeId) -> Vec<HalfEdgeId> {
        let mut cycle = Vec::new();
        let mut current = start;

        loop {
            cycle.push(current);
            current = self.edges[current].next.unwrap();

            if current == start {
                break;
            }
        }

        return cycle;
    }

    /// Gets the destination vertex of a half-edge.
    pub fn get_destination(&self, edge: HalfEdgeId) -> VertexId {
        let twin_id = self.edges[edge].twin.unwrap();
        let origin = self.edges[twin_id].origin;
        return origin;
    }

    /// Makes two half-edges twins.
    pub fn make_twins(&mut self, a: HalfEdgeId, b: HalfEdgeId) {
        (&mut self.edges[a]).twin = Some(b);
        (&mut self.edges[b]).twin = Some(a);
    }

    /// Links edges `a` and `b` sequentially.
    pub fn make_next(&mut self, a: HalfEdgeId, b: HalfEdgeId) {
        (&mut self.edges[a]).next = Some(b);
        (&mut self.edges[b]).prev = Some(a);
    }

    pub fn split_edge_in_half(&mut self, edge: HalfEdgeId) {
        let face = (&self.edges[edge]).face;
        let next = (&self.edges[edge]).next.unwrap();

        let twin = (&self.edges[edge]).twin.unwrap();
        let twin_face = (&self.edges[twin]).face;
        let twin_next = (&self.edges[twin]).next.unwrap();

        // Create vertex for the midpoint
        let mid = {
            let origin = (&self.edges[edge]).origin;
            let dest = (&self.edges[twin]).origin;
            let origin_coord = self.vertices[origin].coord;
            let dest_coord = self.vertices[dest].coord;
            self.add_vertex(midpoint(origin_coord, dest_coord))
        };

        // Create the two new half-edges
        let n1 = self.add_half_edge(mid, face);
        let n2 = self.add_half_edge(mid, twin_face);

        // Set next/prev pointers
        self.make_next(n1, next);
        self.make_next(edge, n1);

        self.make_next(n2, twin_next);
        self.make_next(twin, n2);

        // Set twin pointers
        self.make_twins(twin, n1);
        self.make_twins(edge, n2);
    }

    pub fn remove_inner_edge(&mut self, edge: HalfEdgeId) {
        /* HAVEN'T TESTED THIS YET! */

        let outer_face = self.outer_face;

        let face = (&self.edges[edge]).face;
        let next = (&self.edges[edge]).next.unwrap();
        let prev = (&self.edges[edge]).prev.unwrap();

        let twin = (&self.edges[edge]).twin.unwrap();
        let twin_face = (&self.edges[twin]).face;
        let twin_next = (&self.edges[twin]).next.unwrap();
        let twin_prev = (&self.edges[twin]).prev.unwrap();

        let next_face = (&self.edges[next]).face;
        let twin_next_face = (&self.edges[twin_next]).face;

        if face == outer_face || twin_face == outer_face || face == twin_face {
            return;
        }

        self.make_next(twin_prev, next);
        self.make_next(prev, twin_next);

        self.edges.remove(edge);
        self.edges.remove(twin);
        self.faces.remove(twin_next_face);

        for e in self.get_cycle_from(next) {
            (&mut self.edges[e]).face = next_face;
        }
    }

    pub fn flip_edge(&mut self, edge: HalfEdgeId) {
        unimplemented!()
    }
}