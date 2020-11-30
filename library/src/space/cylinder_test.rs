use demonstrate::demonstrate;

demonstrate! {
    describe "Cylinder" {
        use crate::math::*;
        use crate::lang::ApproximateFloat64Ops;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            let cylinder = Cylinder::default();
        }

        it "A ray misses a cylinder" {
            let examples = [
                // origin    direction
                ((1, 0,  0), (0, 1, 0)),
                ((0, 0,  0), (0, 1, 0)),
                ((0, 0, -5), (1, 1, 1)),
            ];

            for ((ox, oy, oz), (dx, dy, dz)) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                assert_eq!(cylinder.local_intersections(&ray), None);
            }
        }

        it "A ray strikes a cylinder" {
            let examples = [
                // origin    direction      t1       t2
                ((1.0, 0, -5), (0.0, 0, 1), 5.0,     5.0    ),
                ((0.0, 0, -5), (0.0, 0, 1), 4.0,     6.0    ),
                ((0.5, 0, -5), (0.1, 1, 1), 6.80798, 7.08872),
            ];

            for ((ox, oy, oz), (dx, dy, dz), t1, t2) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let (actual_t1, actual_t2) = cylinder.local_intersections(&ray).unwrap();

                assert!(actual_t1.approximate_equals(*t1));
                assert!(actual_t2.approximate_equals(*t2));
            }
        }

        it "Normal vector on a cylinder" {
            let examples = [
                // point       normal
                (( 1,  0,  0), ( 1, 0,  0)),
                (( 0,  5, -1), ( 0, 0, -1)),
                (( 0, -2,  1), ( 0, 0,  1)),
                ((-1,  1,  0), (-1, 0,  0)),
            ];

            for ((px, py, pz), (nx, ny, nz)) in examples.iter() {
                let point = Tuple::point(*px, *py, *pz);
                let expected_normal = Tuple::vector(*nx, *ny, *nz);

                assert_eq!(cylinder.local_normal(&point), expected_normal);
            }
        }
    }
}
