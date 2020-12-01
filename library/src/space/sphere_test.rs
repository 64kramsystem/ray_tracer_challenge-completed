use demonstrate::demonstrate;

demonstrate! {
    describe "Sphere" {
        use crate::space::*;
        use crate::math::*;

        it "should have an identity transformation of order 4 as default" {
            assert_eq!(Sphere::default().transform, Matrix::identity(4));
        }

        it "should allow a transformation to be set" {
            let mut sphere = Sphere::default();
            sphere.transform = Matrix::scaling(1, 2, 3);

            let expected_transform = Matrix::scaling(1, 2, 3);

            assert_eq!(sphere.transform, expected_transform);
        }

        // The property of increasing each id by one can't be tested without modifying the Sphere code.
        // Since a sphere can be initialized everywhere, it's not practical to go and find all the UTs
        // involved.
        // Conditional build attributes can help, but it's still hairy, so it's not worth the hassle.
        //
        it "should return monotonically incrementing ids for each new Sphere" {
            let start_id = Sphere::default().id;

            let next_id = Sphere::default().id;

            assert!(next_id > start_id);

            let next_id_2 = Sphere::default().id;

            assert!(next_id_2 > next_id);
        }

        context "returns the intersections" {
            context "with an untransformed sphere" {
                it "at two points" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(sphere.intersections(&ray), vec![4.0, 6.0]);
                }

                it "at a tangent" {
                    let ray = Ray::new((0, 1, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(sphere.intersections(&ray), vec![5.0, 5.0]);
                }

                it "at no point (miss)" {
                    let ray = Ray::new((0, 2, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(sphere.intersections(&ray), vec![]);
                }
            } // context "with an untransformed sphere"
        } // context "returns the intersections"
    }
}
