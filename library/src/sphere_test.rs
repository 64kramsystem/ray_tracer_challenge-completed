use demonstrate::demonstrate;

demonstrate! {
    describe "Sphere" {
        use crate::tuple::Tuple;
        use crate::matrix::Matrix;
        use crate::sphere::Sphere;

        it "should have an identity transformation of order 4 as default" {
            assert_eq!(Sphere::new().transformation, Matrix::identity(4));
        }

        it "should allow a transformation to be set" {
            let mut sphere = Sphere::new();
            sphere.transformation = Matrix::scaling(1, 2, 3);

            let expected_transformation = Matrix::scaling(1, 2, 3);

            assert_eq!(sphere.transformation, expected_transformation);
        }

        // The property of increasing each id by one can't be tested without modifying the Sphere code.
        // Since a sphere can be initialized everywhere, it's not practical to go and find all the UTs
        // involved.
        // Conditional build attributes can help, but it's still hairy, so it's not worth the hassle.
        //
        it "should return monotonically incrementing ids for each new Sphere" {
            let start_id = Sphere::new().id;

            let next_id = Sphere::new().id;

            assert!(next_id > start_id);

            let next_id_2 = Sphere::new().id;

            assert!(next_id_2 > next_id);
        }

        // See note on the method.
        //
        // it "should return the normal" {
        //     let value = 3.0_f64.powf(-0.5);
        //
        //     let expected_normal = Tuple::vector(value, value, value);
        //
        //     assert_eq!(Sphere::normal(value, value, value), expected_normal);
        // }

        it "should return the normal on a transformed sphere" {
            let sphere = Sphere::new().translate(0, 1, 0);

            let actual_normal = sphere.normal(&Tuple::point(0.0, 1.70711, -0.70711));
            let expected_normal = Tuple::vector(0, 0.70711, -0.70711);

            assert_eq!(actual_normal, expected_normal);
        }
    }
}
