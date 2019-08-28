use mini_obj as obj;
use obj::ObjMesh;


const SAMPLE_DATA: &str = "assets/triangle.obj";


#[test]
fn test_obj_load_file() {
    let result = obj::load_file(SAMPLE_DATA).unwrap();
    let points: Vec<[f32; 3]> = vec![
        [-0.577350, -0.500000, -0.100000], [ 0.577350, -0.500000, -0.100000], [-0.577350, -0.500000,  0.100000],
        [ 0.577350, -0.500000, -0.100000], [ 0.577350, -0.500000,  0.100000], [-0.577350, -0.500000,  0.100000],
        [ 0.577350, -0.500000, -0.100000], [ 0.000000,  0.500000, -0.100000], [ 0.577350, -0.500000,  0.100000],
        [ 0.577350, -0.500000,  0.100000], [ 0.000000,  0.500000, -0.100000], [ 0.000000,  0.500000,  0.100000],
        [-0.577350, -0.500000,  0.100000], [ 0.000000,  0.500000,  0.100000], [-0.577350, -0.500000, -0.100000],
        [-0.577350, -0.500000, -0.100000], [ 0.000000,  0.500000,  0.100000], [ 0.000000,  0.500000, -0.100000],
        [ 0.577350, -0.500000,  0.100000], [ 0.000000,  0.500000,  0.100000], [-0.577350, -0.500000,  0.100000],
        [ 0.577350, -0.500000, -0.100000], [-0.577350, -0.500000, -0.100000], [ 0.000000,  0.500000, -0.100000]
    ]; 
    let tex_coords: Vec<[f32; 2]> = vec![
        [0.000000, 0.000000], [0.000000, 0.000000], [1.000000, 0.000000],
        [0.000000, 0.000000], [1.000000, 1.000000], [1.000000, 0.000000],
        [0.000000, 0.000000], [0.000000, 1.000000], [1.000000, 0.000000],
        [1.000000, 0.000000], [0.000000, 1.000000], [1.000000, 1.000000],
        [1.000000, 1.000000], [0.000000, 1.000000], [1.000000, 0.000000],
        [1.000000, 0.000000], [0.000000, 1.000000], [0.000000, 0.000000],
        [1.000000, 0.000000], [0.500000, 1.000000], [0.000000, 0.000000],
        [0.000000, 0.000000], [1.000000, 0.000000], [0.500000, 1.000000]
    ]; 
    let normals: Vec<[f32; 3]> = vec![
        [ 0.000000, -1.000000,  0.000000], [ 0.000000, -1.000000,  0.000000], [ 0.000000, -1.000000,  0.000000],
        [ 0.000000, -1.000000,  0.000000], [ 0.000000, -1.000000,  0.000000], [ 0.000000, -1.000000,  0.000000],
        [ 0.894400,  0.447200,  0.000000], [ 0.894400,  0.447200,  0.000000], [ 0.894400,  0.447200,  0.000000],
        [ 0.894400,  0.447200,  0.000000], [ 0.894400,  0.447200,  0.000000], [ 0.894400,  0.447200,  0.000000],
        [-0.894400,  0.447200,  0.000000], [-0.894400,  0.447200,  0.000000], [-0.894400,  0.447200,  0.000000],
        [-0.894400,  0.447200,  0.000000], [-0.894400,  0.447200,  0.000000], [-0.894400,  0.447200,  0.000000],
        [ 0.000000,  0.000000,  1.000000], [ 0.000000,  0.000000,  1.000000], [ 0.000000,  0.000000,  1.000000],
        [ 0.000000,  0.000000, -1.000000], [ 0.000000,  0.000000, -1.000000], [ 0.000000,  0.000000, -1.000000]
    ];
    let expected = ObjMesh::new(points, tex_coords, normals);

    assert_eq!(result, expected);
}
