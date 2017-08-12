
extern crate dcel;

// Example super simple use case of DCEL
// currently using as test case becaue `cargo test` will fail if
// examples fail to compile. 

fn main() {

    let mut my_dcel = dcel::api::DCEL::new();
    let face1 = [(0,0), (1,1), (1,0)]; // a right triangle
    let face2 = [(0,0), (1,1), (0,1)]; // another right triangle
    let face3 = [(0,1), (0,2), (1,1)]; // a third right triangle

    // add faces
    let f1_id = my_dcel.add_face(&face1);
    let f2_id = my_dcel.add_face(&face2);
    let f3_id = my_dcel.add_face(&face3);

    assert_eq!(3, my_dcel.num_faces());

    // merge faces
    //my_dcel.merge_faces(f1_id, f2_id); 

}