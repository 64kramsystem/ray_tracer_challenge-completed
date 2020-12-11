use demonstrate::demonstrate;

demonstrate! {
    describe "Csg" {
        use std::sync::Arc;
        // use crate::math::*;
        // use crate::space::{*, shape::private::ShapeLocal};

        use crate::space::*;

        // before {
        //     let plane = Arc::new(Plane::default());
        // }

        it "CSG is created with an operation and two shapes" {
            let sphere1: Arc<dyn Shape> = Arc::new(Sphere::default());
            let sphere2: Arc<dyn Shape> = Arc::new(Cube::default());
            let csg = Arc::new(Csg { operation: csg::Operation::Union, ..Csg::default() });

            csg.set_children(Arc::clone(&sphere1), Arc::clone(&sphere2));

            assert_eq!(csg.operation, csg::Operation::Union);
            assert_eq!(csg.children().0.id(), sphere1.id());
            assert_eq!(csg.children().1.id(), sphere2.id());
            assert_eq!(sphere1.parent().unwrap().id(), csg.id());
            assert_eq!(sphere2.parent().unwrap().id(), csg.id());
        }

        it "Evaluating the rule for a CSG operation" {
            // When result ← intersection_allowed("<op>", <lhit>, <inl>, <inr>)
            // Then result = <result>

            // Examples:
            // | op           | lhit  | inl   | inr   | result |
            // | union        | true  | true  | true  | false  |
            // | union        | true  | true  | false | true   |
            // | union        | true  | false | true  | false  |
            // | union        | true  | false | false | true   |
            // | union        | false | true  | true  | false  |
            // | union        | false | true  | false | false  |
            // | union        | false | false | true  | true   |
            // | union        | false | false | false | true   |
            // # append after the union examples...
            // | intersection | true  | true  | true  | true   |
            // | intersection | true  | true  | false | false  |
            // | intersection | true  | false | true  | true   |
            // | intersection | true  | false | false | false  |
            // | intersection | false | true  | true  | true   |
            // | intersection | false | true  | false | true   |
            // | intersection | false | false | true  | false  |
            // | intersection | false | false | false | false  |
            // # append after the intersection examples...
            // | difference   | true  | true  | true  | false  |
            // | difference   | true  | true  | false | true   |
            // | difference   | true  | false | true  | false  |
            // | difference   | true  | false | false | true   |
            // | difference   | false | true  | true  | true   |
            // | difference   | false | true  | false | true   |
            // | difference   | false | false | true  | false  |
            // | difference   | false | false | false | false  |
        }

        it "Filtering a list of intersections" {
            // Given s1 ← sphere()
            //   And s2 ← cube()
            //   And c ← csg("<operation>", s1, s2)
            //   And xs ← intersections(1:s1, 2:s2, 3:s1, 4:s2)
            // When result ← filter_intersections(c, xs)
            // Then result.count = 2
            //   And result[0] = xs[<x0>]
            //   And result[1] = xs[<x1>]

            // Examples:
            // | operation    | x0 | x1 |
            // | union        | 0  | 3  |
            // | intersection | 1  | 2  |
            // | difference   | 0  | 1  |
        }

        it "A ray misses a CSG object" {
            // Given c ← csg("union", sphere(), cube())
            //   And r ← ray(point(0, 2, -5), vector(0, 0, 1))
            // When xs ← local_intersect(c, r)
            // Then xs is empty
        }

        it "A ray hits a CSG object" {
            // Given s1 ← sphere()
            //   And s2 ← sphere()
            //   And set_transform(s2, translation(0, 0, 0.5))
            //   And c ← csg("union", s1, s2)
            //   And r ← ray(point(0, 0, -5), vector(0, 0, 1))
            // When xs ← local_intersect(c, r)
            // Then xs.count = 2
            //   And xs[0].t = 4
            //   And xs[0].object = s1
            //   And xs[1].t = 6.5
            //   And xs[1].object = s2
        }
    }
}
