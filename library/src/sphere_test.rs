use demonstrate::demonstrate;

demonstrate! {
    describe "Sphere" {
        use crate::sphere::Sphere;

        it "should return consecutive ids for each new Sphere" {
            assert_eq!(Sphere::new().id, 1);
            assert_eq!(Sphere::new().id, 2);
            assert_eq!(Sphere::new().id, 3);
        }
    }
}
