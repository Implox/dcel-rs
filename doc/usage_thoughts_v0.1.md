# Usage Thoughts for version 0.1

Because DCELs are a sophisticated datastructure, the creation of them is something worth spending time considering as there's no standard data format (that I know of - FE) for how their would be stored and thus presented to this library.
Because DCELs are effectively a specialized type of graph which maps to a coordinate system (2D space) there are a few options for how data could be loaded into this library for use:

1. A series of coordinate pairs which contain the coordinates to the end points each edge spans.
2. A list of coordinates for the points and then a list of coordinate ID pairs for how the edges span them.

Both of these present have the same problems in relation to:

1. **Data validity** - How do we know that the graph data is valid? Including:
    1. How do we know there are no crossing edges?
    2. How do we know that all faces (except the exterior face of the graph) are concave, as as classically considered a requirement for a DCEL.
    3. How do we know that all edges form faces? Ex: that there isn't a random line somewhere by its lonesome.
2. **Mutating DCEL over time** - it may be desirable to add or remove vertices and edges over time. This has be be able to be done in a safe manner.

## Possible design

Since changes may need to be made to a DCEL then a "serialized log" sytle construction approach would work best as the construction and mutation of a DCEL are one in the same.
However, to keep things simple and maintain a transactional model for building DCELs, each entry in this log should be a whole operation.
Because the creation of a solo edge or vertice may leave the DCEL in an invalid state each transation should be for the creation (if on the outside perimeter), spliting (if adding edges), or merging (if removing vertices) of edges.

Thus we can imagine a the layers of "importance" as follows (no, I don't know what I mean by "layers of importance" I'm just trying to outline the heirarchy for what justifies what):

1. Faces
2. Edges
3. Points

Because a face belongs to a specific space then the operation to add a new face should look something like this:

```rust
let mut my_dcel = dcel::new();
let face1 = [(0,0), (1,1), (1,0)]; // a right triangle
let face2 = [(0,0), (1,1), (0,1)]; // another right triangle
let face3 = [(0,1), (0,2), (1,1)]; // a third right triangle

// add faces
let f1_id = my_dcel.add_face(face1);
let f2_id = my_dcel.add_face(face2);
let f3_id = my_dcel.add_face(face3);

// merge faces
my_dcel.merge_faces(f1_id, f2_id);
```
