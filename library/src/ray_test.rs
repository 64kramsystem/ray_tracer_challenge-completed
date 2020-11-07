use demonstrate::demonstrate;

demonstrate! {
    describe "Ray" {
        use crate::ray::Ray;
        use crate::tuple::Tuple;

        it "should compute a position at t" {
            let ray = Ray {
                origin: Tuple::point(2, 3, 4),
                direction: Tuple::vector(1, 0, 0),
            };

            assert_eq!(ray.position(0), Tuple::point(2, 3, 4));
            assert_eq!(ray.position(1), Tuple::point(3, 3, 4));
            assert_eq!(ray.position(-1), Tuple::point(1, 3, 4));
            assert_eq!(ray.position(2.5), Tuple::point(4.5, 3, 4));
        }
    }
}
