use std::fmt;
use crate::obj::ObjMesh;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    SymLet,
    SymPoints,
    SymTexCoords,
    SymNormals,
    SymTypeFloat32,
    SymTypeObjMesh,
    SymTypeVec,
    SymConstructor,
    SymMacroVec,
    Equals,
    Colon,
    DoubleColon,
    Semicolon,
    LBracket,
    RBracket,
    LCurlyBrace,
    RCurlyBrace,
    GreaterThan,
    LessThan,
    Comma,
    LParen,
    RParen,
    Float32(f32),
}

#[derive(Clone, Debug, PartialEq)]
struct ObjMeshIR {
    data: Vec<Token>,
}

impl ObjMeshIR {
    fn new(data: Vec<Token>) -> ObjMeshIR {
        ObjMeshIR { data }
    }
}

fn compile(mesh: &ObjMesh) -> ObjMeshIR {
    unimplemented!("Compile has not been implemented yet!");
}

fn synthesize(ir: &ObjMeshIR) -> String {
    unimplemented!("Code synthesis has not been implemented yet!");
}

pub fn to_rust_code(mesh: &ObjMesh) -> String {
    let ir = compile(mesh);
    synthesize(&ir)
}


#[cfg(test)]
mod loader_tests {
    use super::{Token, ObjMeshIR};
    use crate::obj::ObjMesh;
    use crate::obj;
    use std::io::{BufReader, Cursor};

    struct Test {
        obj_file: String,
        obj_mesh: ObjMesh,
        ir: ObjMeshIR,
    }

    fn test() -> Test {
        let obj_file = String::from(r"        \
            o object1                         \
            g cube                            \
            v  0.0  0.0  0.0                  \
            v  0.0  0.0  1.0                  \
            v  0.0  1.0  0.0                  \
            v  0.0  1.0  1.0                  \
            v  1.0  0.0  0.0                  \
            v  1.0  0.0  1.0                  \
            v  1.0  1.0  0.0                  \
            v  1.0  1.0  1.0                  \
                                              \
            vn  0.0  0.0  1.0                 \
            vn  0.0  0.0 -1.0                 \
            vn  0.0  1.0  0.0                 \
            vn  0.0 -1.0  0.0                 \
            vn  1.0  0.0  0.0                 \
            vn -1.0  0.0  0.0                 \
                                              \
            f  1//2  7//2  5//2               \
            f  1//2  3//2  7//2               \
            f  1//6  4//6  3//6               \
            f  1//6  2//6  4//6               \
            f  3//3  8//3  7//3               \
            f  3//3  4//3  8//3               \
            f  5//5  7//5  8//5               \
            f  5//5  8//5  6//5               \
            f  1//4  5//4  6//4               \
            f  1//4  6//4  2//4               \
            f  2//1  6//1  8//1               \
            f  2//1  8//1  4//1               \
        ");
        let points = vec![
            [0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0],
            [0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0],
        ];
        let tex_coords = vec![
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
        ];
        let normals = vec![
            [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
            [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
            [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
            [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
            [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
            [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
            [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
            [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
            [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
            [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
            [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
        ];

        let obj_mesh = ObjMesh {
            points: points,
            tex_coords: tex_coords,
            normals: normals,
        };

        use Token::*; 
        let ir = ObjMeshIR::new(vec![
            LCurlyBrace,
                SymLet, SymPoints, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                RBracket, Semicolon,
                SymLet, SymTexCoords, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                RBracket, Semicolon,
                SymLet, SymNormals, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,

                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                RBracket, Semicolon,
                SymTypeObjMesh, DoubleColon, SymConstructor, LParen, 
                    SymPoints, Comma, SymTexCoords, Comma, SymNormals, 
                RParen,
            RCurlyBrace,
        ]);

        Test {
            obj_file: obj_file,
            obj_mesh: obj_mesh,
            ir: ir,
        }
    }

    #[test]
    fn test_compile_obj_mesh() {
        let test = test();
        let mut reader = BufReader::new(Cursor::new(test.obj_file.as_bytes()));
        let result = obj::load(&mut reader).unwrap();
        let expected = test.obj_mesh;

        assert_eq!(result, expected);
    }
}