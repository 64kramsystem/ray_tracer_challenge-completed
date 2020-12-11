use demonstrate::demonstrate;

demonstrate! {
    describe "Shape" {
        use crate::Axis;
        use crate::lang::math::sqrt;
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

            let actual_normal = sphere.normal(&Tuple::point(0.0, 1.70711, -0.70711), &Intersection::default());
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
                    let test_shape: Arc<dyn Shape> = Arc::new(test_shape.scale(2, 2, 2));

                    let actual_intersections = test_shape.intersections(&ray);

                    assert_eq!(actual_intersections.len(), 2);

                    assert_eq!(actual_intersections[0].t, 3.0);
                    assert_eq!(actual_intersections[1].t, 7.0);
                }

                it "translated" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let test_shape: Arc<dyn Shape> = Arc::new(test_shape.translate(5, 0, 0));

                    assert_eq!(test_shape.intersections(&ray), vec![]);
                }
            } // context "with a transformed shape"
        }

        it "Converting a point from world to object space" {
            let group1: Arc<Group> = Arc::new(Group {
                transform: Matrix::rotation(Axis::Y, PI / 2.0),
                ..Group::default()
            });

            let group2: Arc<Group> = Arc::new(Group {
                transform: Matrix::scaling(2, 2, 2),
                ..Group::default()
            });

            group1.add_child(&(Arc::clone(&group2) as Arc<dyn Shape>));

            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            group2.add_child(&sphere);

            let expected_point = Tuple::point(0, 0, -1);

            assert_eq!(sphere.world_to_object(&Tuple::point(-2, 0, -10)), expected_point);
        }

        it "Converting a normal from object to world space" {
            let group1: Arc<Group> = Arc::new(Group {
                transform: Matrix::rotation(Axis::Y, PI / 2.0),
                ..Group::default()
            });

            let group2: Arc<Group> = Arc::new(Group {
                transform: Matrix::scaling(1, 2, 3),
                ..Group::default()
            });

            group1.add_child(&(Arc::clone(&group2) as Arc<dyn Shape>));

            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            group2.add_child(&sphere);

            let actual_normal = sphere.normal_to_world(&Tuple::vector(sqrt(3) / 3.0, sqrt(3) / 3.0, sqrt(3) / 3.0));

            assert_eq!(actual_normal, Tuple::vector(0.2857, 0.4286, -0.8571));
        }

        it "Finding the normal on a child object" {
            let group1: Arc<Group> = Arc::new(Group {
                transform: Matrix::rotation(Axis::Y, PI / 2.0),
                ..Group::default()
            });

            let group2: Arc<Group> = Arc::new(Group {
                transform: Matrix::scaling(1, 2, 3),
                ..Group::default()
            });

            group1.add_child(&(Arc::clone(&group2) as Arc<dyn Shape>));

            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            group2.add_child(&sphere);

            let actual_normal = sphere.normal(&Tuple::point(1.7321, 1.1547, -5.5774), &Intersection::default());

            assert_eq!(actual_normal, Tuple::vector(0.2857, 0.4286, -0.8571));
        }
    }
}
