use demonstrate::demonstrate;

demonstrate! {
    describe "RingPattern" {
        use crate::math::*;
        use crate::properties::*;

        it "should extend in both x and z" {
            let pattern = RingPattern::default();

            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(1, 0, 0)), COLOR_BLACK);
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 1)), COLOR_BLACK);
            // 0.708 = just slightly more than √2/2
            assert_eq!(pattern.color_at(&Tuple::point(0.708, 0, 0.708)), COLOR_BLACK);
        }
    }
}
