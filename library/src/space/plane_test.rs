use demonstrate::demonstrate;

demonstrate! {
    describe "Plane" {
        use std::sync::Arc;
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            let plane = Arc::new(Plane::default());
        }

        it "should have a constant normal, everywhere" {
            let expected_normal = Tuple::vector(0, 1, 0);

            assert_eq!(plane.local_normal(&Tuple::point(0, 0, 0)), expected_normal);
            assert_eq!(plane.local_normal(&Tuple::point(10, 0, -10)), expected_normal);
            assert_eq!(plane.local_normal(&Tuple::point(-5, 0, 150)), expected_normal);
        }

        context "intersections" {
            context "should be absent"  {
                it "with a ray parallel to the plane" {
                    let ray = Ray::new((0, 10, 0), (0, 0, 1));
                    let intersections = plane.local_intersections(&ray);
                    assert_eq!(intersections, vec![]);
                }

                it "with a coplanar ray" {
                    let ray = Ray::new((0, 0, 0), (0, 0, 1));
                    let intersections = plane.local_intersections(&ray);
                    assert_eq!(intersections, vec![]);
                }
            } // context "should be absent"

            context "should be present"  {
                it "with a plane from above" {
                    let ray = Ray::new((0, 1, 0), (0, -1, 0));
                    let actual_intersections = plane.local_intersections(&ray);

                    assert_eq!(actual_intersections.len(), 1);
                    assert_eq!(actual_intersections[0].t, 1.0);
                }

                it "with a plane from below" {
                    let ray = Ray::new((0, -1, 0), (0, 1, 0));
                    let actual_intersections = plane.local_intersections(&ray);

                    assert_eq!(actual_intersections.len(), 1);
                    assert_eq!(actual_intersections[0].t, 1.0);
                }
            } // context "should be absent"
        } // context "intersections"
    }
}
