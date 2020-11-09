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

        // There's no simple solution to this. The `serial_test` crate can't be used with `demonstrate!`.
        // If we make the lock public, we still can't acquire it inside the UT, because the acquisition
        // inside Sphere::new() will wait.
        // Therefore, without adhoc modifications (which are not worth), it seems that we can't test the
        // increase by one unit.
        //
        // Unfortunately, this must be made public for testing purposes; the new() method can be invoked from
        // anywhere, so there's no way to isolate UTs requiring locking.
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

            let actual_normal = sphere.normal(Tuple::point(0.0, 1.70711, -0.70711));
            let expected_normal = Tuple::vector(0, 0.70711, -0.70711);

            assert_eq!(actual_normal, expected_normal);
        }
    }
}
