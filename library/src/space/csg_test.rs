use demonstrate::demonstrate;

demonstrate! {
    describe "Csg" {
        use std::sync::Arc;
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal, csg};

        // before {
        //     let plane = Arc::new(Plane::default());
        // }

        it "CSG is created with an operation and two shapes" {
            let sphere1: Arc<dyn Shape> = Arc::new(Sphere::default());
            let sphere2: Arc<dyn Shape> = Arc::new(Cube::default());
            let csg = Csg::new(
                csg::Operation::Union,
                Arc::clone(&sphere1),
                Arc::clone(&sphere2),
                Matrix::identity(4)
            );

            assert_eq!(csg.operation, csg::Operation::Union);
            assert_eq!(csg.children.0.id(), sphere1.id());
            assert_eq!(csg.children.1.id(), sphere2.id());
            assert_eq!(sphere1.parent().unwrap().id(), csg.id());
            assert_eq!(sphere2.parent().unwrap().id(), csg.id());
        }

        it "Evaluating the rule for a CSG operation" {
            let examples = vec![
                // op                          lhit                   inl     inr     result
                (csg::Operation::Union,        csg::ChildHit::Left,   true,   true,   false),
                (csg::Operation::Union,        csg::ChildHit::Left,   true,   false,  true),
                (csg::Operation::Union,        csg::ChildHit::Left,   false,  true,   false),
                (csg::Operation::Union,        csg::ChildHit::Left,   false,  false,  true),
                (csg::Operation::Union,        csg::ChildHit::Right,  true,   true,   false),
                (csg::Operation::Union,        csg::ChildHit::Right,  true,   false,  false),
                (csg::Operation::Union,        csg::ChildHit::Right,  false,  true,   true),
                (csg::Operation::Union,        csg::ChildHit::Right,  false,  false,  true),
                (csg::Operation::Intersection, csg::ChildHit::Left,   true,   true,   true),
                (csg::Operation::Intersection, csg::ChildHit::Left,   true,   false,  false),
                (csg::Operation::Intersection, csg::ChildHit::Left,   false,  true,   true),
                (csg::Operation::Intersection, csg::ChildHit::Left,   false,  false,  false),
                (csg::Operation::Intersection, csg::ChildHit::Right,  true,   true,   true),
                (csg::Operation::Intersection, csg::ChildHit::Right,  true,   false,  true),
                (csg::Operation::Intersection, csg::ChildHit::Right,  false,  true,   false),
                (csg::Operation::Intersection, csg::ChildHit::Right,  false,  false,  false),
                (csg::Operation::Difference,   csg::ChildHit::Left,   true,   true,   false),
                (csg::Operation::Difference,   csg::ChildHit::Left,   true,   false,  true),
                (csg::Operation::Difference,   csg::ChildHit::Left,   false,  true,   false),
                (csg::Operation::Difference,   csg::ChildHit::Left,   false,  false,  true),
                (csg::Operation::Difference,   csg::ChildHit::Right,  true,   true,   true),
                (csg::Operation::Difference,   csg::ChildHit::Right,  true,   false,  true),
                (csg::Operation::Difference,   csg::ChildHit::Right,  false,  true,   false),
                (csg::Operation::Difference,   csg::ChildHit::Right,  false,  false,  false),
            ];

            for (operation, child_hit, in_left, in_right, expected_result) in examples.into_iter() {
                let csg = Arc::new(Csg { operation, ..Csg::default() });
                let actual_result = csg.intersection_allowed(child_hit, in_left, in_right);

                assert_eq!(actual_result, expected_result);
            }
        }

        it "Filtering a list of intersections" {
            let examples = vec![
                // operation                    x0 x1
                (csg::Operation::Union,         0, 3),
                (csg::Operation::Intersection,  1, 2),
                (csg::Operation::Difference,    0, 1),
            ];

            let left: Arc<dyn Shape> = Arc::new(Sphere::default());
            let right: Arc<dyn Shape> = Arc::new(Cube::default());

            let intersections = vec![
                Intersection { t: 1.0, uv: None, object: left.as_ref() },
                Intersection { t: 2.0, uv: None, object: right.as_ref() },
                Intersection { t: 3.0, uv: None, object: left.as_ref() },
                Intersection { t: 4.0, uv: None, object: right.as_ref() },
            ];

            for (operation, x0, x1) in examples.into_iter() {
                let csg = Csg::new(
                    operation,
                    Arc::clone(&left),
                    Arc::clone(&right),
                    Matrix::identity(4)
                );

                let result = csg.filter_intersections(intersections.clone());

                assert_eq!(result.len(), 2);

                assert_eq!(result[0], intersections[x0]);
                assert_eq!(result[1], intersections[x1]);
            }
        }

        it "A ray misses a CSG object" {
            let csg = Csg::new(
                csg::Operation::Union,
                Arc::new(Sphere::default()),
                Arc::new(Cube::default()),
                Matrix::identity(4),
            );

            let ray = Ray::new((0, 2, -5), (0, 0, 1));

            let intersections = csg.local_intersections(&ray);

            assert_eq!(intersections.len(), 0);
        }

        it "A ray hits a CSG object" {
            let s1: Arc<dyn Shape> = Arc::new(Sphere::default());
            let s2: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(0.0, 0.0, 0.5),
                ..Sphere::default()
            });

            let csg = Csg::new(
                csg::Operation::Union,
                Arc::clone(&s1),
                Arc::clone(&s2),
                Matrix::identity(4),
            );

            let ray = Ray::new((0, 0, -5), (0, 0, 1));

            let intersections = csg.local_intersections(&ray);

            assert_eq!(intersections.len(), 2);
            assert_eq!(intersections[0].t, 4.0);
            assert_eq!(intersections[0].object, s1.as_ref());
            assert_eq!(intersections[1].t, 6.5);
            assert_eq!(intersections[1].object, s2.as_ref());
        }
    }
}
