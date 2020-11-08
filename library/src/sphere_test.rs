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

        it "should return consecutive ids for each new Sphere" {
            let start_id = Sphere::new().id;

            assert_eq!(Sphere::new().id, start_id + 1);
            assert_eq!(Sphere::new().id, start_id + 2);
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
