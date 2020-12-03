use demonstrate::demonstrate;

demonstrate! {
    describe "Shape" {
        use crate::Axis;
        use crate::math::*;
        use crate::space::*;
        use std::sync::Arc;
        use std::f64::consts::PI;

        before {
            #[allow(unused_variables)]
            let test_shape = Sphere::default();
        }

        it "should return the normal on a transformed sphere" {
            let sphere: Box<dyn Shape> = Box::new(test_shape.translate(0, 1, 0));

            let actual_normal = sphere.normal(&Tuple::point(0.0, 1.70711, -0.70711));
            let expected_normal = Tuple::vector(0, 0.70711, -0.70711);

            assert_eq!(actual_normal, expected_normal);
        }

        // The `Shape#saved_ray` field is mysterious, as it's not used anymore in the book.
        // Implementing it is also not trivial, because it triggers a cascade of &mut changes,
        // due to Shape#intersections being a core API.
        // For this reason, `saved_ray()` hasn't been implemented, and these UTs have just been moved
        // and generalized from the Sphere test suite.
        // Finally, didn't bother to move other UTs.
        //
        context "returns the intersections" {
            context "with a transformed shape" {
                it "scaled" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let test_shape: Box<dyn Shape> = Box::new(test_shape.scale(2, 2, 2));

                    assert_eq!(test_shape.intersections(&ray), vec![3.0, 7.0]);
                }

                it "translated" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let test_shape: Box<dyn Shape> = Box::new(test_shape.translate(5, 0, 0));

                    assert_eq!(test_shape.intersections(&ray), vec![]);
                }
            } // context "with a transformed shape"
        }

        it "Converting a point from world to object space" {
            let group1: Arc<dyn Shape> = Arc::new(Group {
                transform: Matrix::rotation(Axis::Y, PI / 2.0),
                ..Group::default()
            });

            let group2: Arc<dyn Shape> = Arc::new(Group {
                transform: Matrix::scaling(2, 2, 2),
                ..Group::default()
            });

            Group::add_child(&group1, &group2);

            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            Group::add_child(&group2, &sphere);

            let expected_point = Tuple::point(0, 0, -1);

            assert_eq!(sphere.world_to_object(&Tuple::point(-2, 0, -10)), expected_point);
        }
    }
}
