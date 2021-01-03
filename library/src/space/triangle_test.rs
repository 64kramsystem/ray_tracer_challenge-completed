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
            let default_intersection = Intersection {
                t: 0.0,
                uv: None,
                object: &Plane::default(),
            };
            let t = Triangle::from_ints((0, 1, 0), (-1, 0, 0), (1, 0, 0));
            let n1 = t.local_normal(Tuple::point(0, 0.5, 0), &default_intersection);
            let n2 = t.local_normal(Tuple::point(-0.5, 0.75, 0), &default_intersection);
            let n3 = t.local_normal(Tuple::point(0.5, 0.25, 0), &default_intersection);

            assert_eq!(n1, t.normal);
            assert_eq!(n2, t.normal);
            assert_eq!(n3, t.normal);
        }
    } // describe "Triangle"

    describe "Smooth triangle" {
        use crate::lang::ApproximateFloat64Ops;
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            let p1 = Tuple::point(0, 1, 0);
            let p2 = Tuple::point(-1, 0, 0);
            let p3 = Tuple::point(1, 0, 0);
            let n1 = Tuple::vector(0, 1, 0);
            let n2 = Tuple::vector(-1, 0, 0);
            let n3 = Tuple::vector(1, 0, 0);

            let triangle = Triangle::smooth(p1, p2, p3, n1, n2, n3);
        }

        it "Constructing a smooth triangle" {
            assert_eq!(triangle.p1, p1);
            assert_eq!(triangle.p2, p2);
            assert_eq!(triangle.p3, p3);
            assert_eq!(triangle.vertex_normals, Some((n1, n2, n3)));
        }

        it "An intersection with a smooth triangle stores u/v" {
            let ray = Ray::new((-0.2, 0.3, -2.0), (0, 0, 1));

            let intersections = triangle.local_intersections(&ray);

            let actual_uv = intersections[0].uv.unwrap();

            assert!(actual_uv.0.approximate_equals(0.45));
            assert!(actual_uv.1.approximate_equals(0.25));
        }

        it "A smooth triangle uses u/v to interpolate the normal" {
            let intersection = Intersection { t: 1.0, uv: Some((0.45, 0.25)), object: &triangle };

            // Requires `i`
            // And n ← normal_at(tri, point(0, 0, 0), i)
            //
            let normal = intersection.object.normal(&Tuple::point(0, 0, 0), &intersection);

            assert_eq!(normal, Tuple::vector(-0.5547, 0.83205, 0));
        }

        it "Preparing the normal on a smooth triangle" {
            let intersections = [Intersection { t: 1.0, uv: Some((0.45, 0.25)), object: &triangle }];
            let ray = Ray::new((-0.2, 0.3, -2.0), (0, 0, 1));

            let comps = ray.intersection_state(&intersections[0], &intersections);

            assert_eq!(comps.normalv, Tuple::vector(-0.5547, 0.83205, 0));
        }
    } // describe "Smooth triangle"
}
