use demonstrate::demonstrate;

const ASSETS_PATH: &str = "../testing_assets";

demonstrate! {
    describe "ObjParser" {
        use super::ASSETS_PATH;
        use indoc::indoc;
        use crate::utils::ObjParser;
        use crate::math::Tuple;
        use crate::space::Triangle;
        use std::sync::Arc;
        use std::{io::BufReader, fs::File, path::Path};

        it "Ignoring unrecognized lines" {
            let input = indoc! {"
                There was a young lady named Bright
                who traveled much faster than light.
                She set out one day
                in a relative way,
                and came back the previous night.
            "};

            // Just test that the lines don't cause an error.
            //
            ObjParser::parse(input.as_bytes()).unwrap();
        }

        it "Vertex records" {
            let input = indoc! {"
                v -1 1 0
                v -1.0000 0.5000 0.0000
                v 1 0 0
                v 1 1 0
            "};

            let parser = ObjParser::parse(input.as_bytes()).unwrap();

            assert_eq!(parser.vertex(1), Tuple::point(-1, 1, 0));
            assert_eq!(parser.vertex(2), Tuple::point(-1, 0.5, 0));
            assert_eq!(parser.vertex(3), Tuple::point(1, 0, 0));
            assert_eq!(parser.vertex(4), Tuple::point(1, 1, 0));
        }

        it "Parsing triangle faces" {
            let input = indoc! {"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0

                f 1 2 3
                f 1 3 4
            "};

            let parser = ObjParser::parse(input.as_bytes()).unwrap();

            let group = &parser.default_group();

            let children = &group.children().iter().map(|child| Arc::clone(&child)).collect::<Vec<_>>();

            let t1 = children[0].as_any().downcast_ref::<Triangle>().unwrap();
            let t2 = children[1].as_any().downcast_ref::<Triangle>().unwrap();

            assert_eq!(t1.p1, parser.vertex(1));
            assert_eq!(t1.p2, parser.vertex(2));
            assert_eq!(t1.p3, parser.vertex(3));
            assert_eq!(t2.p1, parser.vertex(1));
            assert_eq!(t2.p2, parser.vertex(3));
            assert_eq!(t2.p3, parser.vertex(4));
        }

        it "Triangulating polygons" {
            let input = indoc! {"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0
                v 0 2 0

                f 1 2 3 4 5
            "};

            let parser = ObjParser::parse(input.as_bytes()).unwrap();

            let group = &parser.default_group();

            let children = &group.children().iter().map(|child| Arc::clone(&child)).collect::<Vec<_>>();

            let t1 = children[0].as_any().downcast_ref::<Triangle>().unwrap();
            let t2 = children[1].as_any().downcast_ref::<Triangle>().unwrap();
            let t3 = children[2].as_any().downcast_ref::<Triangle>().unwrap();

            assert_eq!(t1.p1, parser.vertex(1));
            assert_eq!(t1.p2, parser.vertex(2));
            assert_eq!(t1.p3, parser.vertex(3));
            assert_eq!(t2.p1, parser.vertex(1));
            assert_eq!(t2.p2, parser.vertex(3));
            assert_eq!(t2.p3, parser.vertex(4));
            assert_eq!(t3.p1, parser.vertex(1));
            assert_eq!(t3.p2, parser.vertex(4));
            assert_eq!(t3.p3, parser.vertex(5));
        }

        it "Triangles in groups" {
            let file_path = Path::new(ASSETS_PATH).join("triangles.obj");
            let file_reader = BufReader::new(File::open(file_path).unwrap());

            let parser = ObjParser::parse(file_reader).unwrap();

            let group1 = parser.group("FirstGroup");
            let group2 = parser.group("SecondGroup");

            let t1 = Arc::clone(&group1.children()[0]);
            let t2 = Arc::clone(&group2.children()[0]);

            let t1 = t1.as_any().downcast_ref::<Triangle>().unwrap();
            let t2 = t2.as_any().downcast_ref::<Triangle>().unwrap();

            assert_eq!(t1.p1, parser.vertex(1));
            assert_eq!(t1.p2, parser.vertex(2));
            assert_eq!(t1.p3, parser.vertex(3));
            assert_eq!(t2.p1, parser.vertex(1));
            assert_eq!(t2.p2, parser.vertex(3));
            assert_eq!(t2.p3, parser.vertex(4));
        }

        // it "Converting an OBJ file to a group" {
        // Given file ← the file "triangles.obj"
        //     And parser ← parse_obj_file(file)
        // When g ← obj_to_group(parser)
        // Then g includes "FirstGroup" from parser
        //     And g includes "SecondGroup" from parser
        // }

        // it "Vertex normal records" {
        // let input = indoc! {"
        //     """
        //     vn 0 0 1
        //     vn 0.707 0 -0.707
        //     vn 1 2 3
        //     """
        // let parser = parse_obj_file(file)
        // Then parser.normals[1] = vector(0, 0, 1)
        //     And parser.normals[2] = vector(0.707, 0, -0.707)
        //     And parser.normals[3] = vector(1, 2, 3)
        // }

        // it "Faces with normals" {
        // let input = indoc! {"
        //     """
        //     v 0 1 0
        //     v -1 0 0
        //     v 1 0 0

        //     vn -1 0 0
        //     vn 1 0 0
        //     vn 0 1 0

        //     f 1//3 2//1 3//2
        //     f 1/0/3 2/102/1 3/14/2
        //     """
        // let parser = parse_obj_file(file)
        //     And g ← parser.default_group
        //     And t1 ← first child of g
        //     And t2 ← second child of g
        // Then t1.p1 = parser.vertices[1]
        //     And t1.p2 = parser.vertices[2]
        //     And t1.p3 = parser.vertices[3]
        //     And t1.n1 = parser.normals[3]
        //     And t1.n2 = parser.normals[1]
        //     And t1.n3 = parser.normals[2]
        //     And t2 = t1
        // }
    }
}
