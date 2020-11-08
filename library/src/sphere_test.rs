use demonstrate::demonstrate;

demonstrate! {
    describe "Sphere" {
        use crate::matrix::Matrix;
        use crate::sphere::Sphere;

        it "should have an identity transformation of order 4 as default" {
            assert_eq!(Sphere::new(None).transformation, Matrix::identity(4));
        }

        it "should allow a transformation to be set" {
            let expected_transformation = Matrix::scaling(1, 2, 3);

            assert_eq!(Sphere::new(Some(Matrix::scaling(1, 2, 3))).transformation, expected_transformation);
        }

        it "should return consecutive ids for each new Sphere" {
            let start_id = Sphere::new(None).id;

            assert_eq!(Sphere::new(None).id, start_id + 1);
            assert_eq!(Sphere::new(None).id, start_id + 2);
        }
    }
}
