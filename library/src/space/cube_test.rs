use demonstrate::demonstrate;

demonstrate! {
    describe "Cube" {
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            let cube = Cube::default();
        }

        it "a ray intersects a cube" {
            let examples = [
               // origin            direction      t1   t2
               (( 5.0,  0.5,  0.0), (-1,  0,  0),  4.0, 6.0), // +x
               ((-5.0,  0.5,  0.0), ( 1,  0,  0),  4.0, 6.0), // -x
               (( 0.5,  5.0,  0.0), ( 0, -1,  0),  4.0, 6.0), // +y
               (( 0.5, -5.0,  0.0), ( 0,  1,  0),  4.0, 6.0), // -y
               (( 0.5,  0.0,  5.0), ( 0,  0, -1),  4.0, 6.0), // +z
               (( 0.5,  0.0, -5.0), ( 0,  0,  1),  4.0, 6.0), // -z
               (( 0.0,  0.5,  0.0), ( 0,  0,  1), -1.0, 1.0), // inside
            ];

            for ((ox, oy, oz), (dx, dy, dz), t1, t2) in examples.iter() {
                let ray = Ray::new((*ox, *oy, *oz), (*dx, *dy, *dz));

                let expected_intersections = vec![*t1, *t2];

                assert_eq!(cube.local_intersections(&ray), expected_intersections);
            }
        }

        it "a ray misses a cube" {
            let examples = [
                // origin,                  direction
                ((-2,  0,  0), ( 0.2673,  0.5345,  0.8018)),
                (( 0, -2,  0), ( 0.8018,  0.2673,  0.5345)),
                (( 0,  0, -2), ( 0.5345,  0.8018,  0.2673)),
                (( 2,  0,  2), ( 0.0,     0.0,    -1.0   )),
                (( 0,  2,  2), ( 0.0,    -1.0,     0.0   )),
                (( 2,  2,  0), (-1.0,     0.0,     0.0   )),
            ];

            for ((ox, oy, oz), (dx, dy, dz)) in examples.iter() {
                let ray = Ray {
                    origin: Tuple::point(*ox, *oy, *oz),
                    direction: Tuple::vector(*dx, *dy, *dz),
                };

                assert_eq!(cube.local_intersections(&ray), vec![]);
            }
        }

        it "The normal on the surface of a cube" {
            // And p ← <point>
            // When normal ← local_normal_at( c, p)
            // Then normal = <normal>

            let examples = [
                // point             normal
                // (( 1.0,  0.5, -0.8), ( 1.0,  0.0,  0.0)),
                ((-1.0, -0.2,  0.9), (-1.0,  0.0,  0.0)),
                // ((-0.4,  1.0, -0.1), ( 0.0,  1.0,  0.0)),
                // (( 0.3, -1.0, -0.7), ( 0.0, -1.0,  0.0)),
                // ((-0.6,  0.3,  1.0), ( 0.0,  0.0,  1.0)),
                // (( 0.4,  0.4, -1.0), ( 0.0,  0.0, -1.0)),
                // (( 1.0,  1.0,  1.0), ( 1.0,  0.0,  0.0)),
                // ((-1.0, -1.0, -1.0), (-1.0,  0.0,  0.0)),
            ];

            for ((px, py, pz), (nx, ny, nz)) in examples.iter() {
                let point = Tuple::point(*px, *py, *pz);
                let expected_normal = Tuple::vector(*nx, *ny, *nz);

                assert_eq!(cube.local_normal(&point), expected_normal);
            }
        }
    }
}
