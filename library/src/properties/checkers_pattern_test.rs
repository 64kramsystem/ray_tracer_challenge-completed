use demonstrate::demonstrate;

demonstrate! {
    describe "CheckersPattern" {
        use crate::math::*;
        use crate::properties::*;

        before {
            let pattern = CheckersPattern::default();
        }

        it "should repeat in x" {
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0.99, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(1.01, 0, 0)), COLOR_BLACK);
        }

        it "should repeat in y" {
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 0.99, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 1.01, 0)), COLOR_BLACK);
        }

        it "should repeat in z" {
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0.99)), COLOR_WHITE);
            assert_eq!(pattern.color_at(&Tuple::point(0, 0, 1.01)), COLOR_BLACK);
        }
    }
}
