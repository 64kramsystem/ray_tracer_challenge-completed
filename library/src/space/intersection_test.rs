use demonstrate::demonstrate;

demonstrate! {
    describe "Intersection" {
        use crate::math::*;
        use crate::space::*;

        it "An intersection can encapsulate `u` and `v`" {
            let triangle = Triangle::new(Tuple::point(0, 1, 0), Tuple::point(-1, 0, 0), Tuple::point(1, 0, 0));

            let intersection = Intersection { t: 3.5, uv: Some((0.2, 0.4)), object: &triangle };

            assert_eq!(intersection.uv, Some((0.2, 0.4)));
        }
    }
}
