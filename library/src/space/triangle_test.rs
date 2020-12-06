use demonstrate::demonstrate;

demonstrate! {
    describe "Triangle" {
        use std::sync::Arc;
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};

        it "Constructing a triangle" {
            let t = Triangle::from_ints(
                (0, 1, 0),
                (-1, 0, 0),
                (1, 0, 0),
            );

            assert_eq!(t.p1, Tuple::point(0, 1, 0));
            assert_eq!(t.p2, Tuple::point(-1, 0, 0));
            assert_eq!(t.p3, Tuple::point(1, 0, 0));
            assert_eq!(t.e1, Tuple::vector(-1, -1, 0));
            assert_eq!(t.e2, Tuple::vector(1, -1, 0));
            assert_eq!(t.normal, Tuple::vector(0, 0, -1));
        }

        it "Intersecting a ray parallel to the triangle" {
            let t = Arc::new(Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0)));
            let r = Ray::new((0, -1, -2), (0, 1, 0));
            let xs = t.local_intersections(&r);

            assert_eq!(xs.len(), 0);
        }

        it "A ray misses the p1-p3 edge" {
            let t = Arc::new(Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0)));
            let r = Ray::new((1, 1, -2), (0, 0, 1));
            let xs = t.local_intersections(&r);

            assert_eq!(xs.len(), 0);
        }

        it "A ray misses the p1-p2 edge" {
            let t = Arc::new(Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0)));
            let r = Ray::new((-1, 1, -2), (0, 0, 1));
            let xs = t.local_intersections(&r);

            assert_eq!(xs.len(), 0);
        }

        it "A ray misses the p2-p3 edge" {
            let t = Arc::new(Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0)));
            let r = Ray::new((0, -1, -2), (0, 0, 1));
            let xs = t.local_intersections(&r);

            assert_eq!(xs.len(), 0);
        }

        it "A ray strikes a triangle" {
          let t = Arc::new(Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0)));
            let r = Ray::new((0.0, 0.5, -2.0), (0, 0, 1));
            let xs = t.local_intersections(&r);

            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 2.0);
        }

        it "Finding the normal on a triangle" {
            let t = Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0));
            let n1 = t.local_normal(&Tuple::point(0, 0.5, 0));
            let n2 = t.local_normal(&Tuple::point(-0.5, 0.75, 0));
            let n3 = t.local_normal(&Tuple::point(0.5, 0.25, 0));

            assert_eq!(n1, t.normal);
            assert_eq!(n2, t.normal);
            assert_eq!(n3, t.normal);
        }
    } // describe "Triangle"
}
