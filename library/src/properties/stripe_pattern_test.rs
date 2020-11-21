use demonstrate::demonstrate;

demonstrate! {
    describe "StripePattern" {
        use crate::math::*;
        use crate::properties::*;

        before {
            let pattern = StripePattern::default();
        }

        it "is constant in y" {
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 1, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 2, 0)), COLOR_WHITE);
        }

        it "is constant in z" {
                assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
                assert_eq!(pattern.color_at(&Tuple::point(0, 0, 1)), COLOR_WHITE);
                assert_eq!(pattern.color_at(&Tuple::point(0, 0, 2)), COLOR_WHITE);
        }

        it "alternates in x" {
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0.9, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(1, 0, 0)), COLOR_BLACK);
            assert_eq!(pattern.color_at(&Tuple::point(-0.1, 0, 0)), COLOR_BLACK);
            assert_eq!(pattern.color_at(&Tuple::point(-1, 0, 0)), COLOR_BLACK);
            assert_eq!(pattern.color_at(&Tuple::point(-1.1, 0, 0)), COLOR_WHITE);
        }
    }
}
