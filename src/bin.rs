extern crate libdcel;

use libdcel::geometry::*;
use libdcel::dcel::*;

pub fn edge_split() {
    let mut dcel = DCEL::new();

    let p1 = Point2::new(0.0, 0.0);
    let p2 = Point2::new(1.0, 0.0);
    let p3 = Point2::new(0.0, 1.0);

    let v1 = dcel.add_vertex(p1);
    let v2 = dcel.add_vertex(p2);
    let v3 = dcel.add_vertex(p3);

    let inner_face = dcel.add_face();
    let outer_face = dcel.outer_face;

    let e12 = dcel.add_half_edge(v1, inner_face);
    let e23 = dcel.add_half_edge(v2, inner_face);
    let e31 = dcel.add_half_edge(v3, inner_face);

    let e13 = dcel.add_half_edge(v1, outer_face);
    let e32 = dcel.add_half_edge(v3, outer_face);
    let e21 = dcel.add_half_edge(v2, outer_face);

    dcel.make_twins(e12, e21);
    dcel.make_twins(e23, e32);
    dcel.make_twins(e31, e13);

    dcel.make_next(e12, e23);
    dcel.make_next(e23, e31);
    dcel.make_next(e31, e12);

    dcel.make_next(e13, e32);
    dcel.make_next(e32, e21);
    dcel.make_next(e21, e13);

    println!("{:?}\n", dcel);
    println!("{:?}", dcel.get_cycle_from(e12));
    println!("{:?}", dcel.get_cycle_from(e13));

    dcel.split_edge_in_half(e12);
    println!("{:?}", dcel.get_cycle_from(e12));
    println!("{:?}", dcel.get_cycle_from(e13));
}

pub fn main() {
    //edge_split();
}