use demonstrate::demonstrate;

demonstrate! {
    describe "GradientPattern" {
        use crate::math::*;
        use crate::properties::*;

        it "should interpolate linearly between colors" {
            let pattern = GradientPattern::default();
              assert_eq!(pattern.color_at(&Tuple::point(-0.25, 0, 0)), COLOR_BLACK);
              assert_eq!(pattern.color_at(&Tuple::point(0, 0, 0)), Color::new(0.5, 0.5, 0.5));
              assert_eq!(pattern.color_at(&Tuple::point(0.25, 0, 0)), COLOR_WHITE);
              assert_eq!(pattern.color_at(&Tuple::point(0.5, 0, 0)), Color::new(0.5, 0.5, 0.5));
              assert_eq!(pattern.color_at(&Tuple::point(0.75, 0, 0)), COLOR_BLACK);
              assert_eq!(pattern.color_at(&Tuple::point(1.0, 0, 0)), Color::new(0.5, 0.5, 0.5));
        }
    }
}
