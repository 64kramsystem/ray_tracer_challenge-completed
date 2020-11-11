use std::collections::BTreeSet;

use crate::{intersection::Intersection, Color, IntersectionState, PointLight, Ray, Sphere, Tuple};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: PointLight,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light_source: PointLight {
                position: Tuple::point(-10, 10, -10),
                intensity: Color::new(1, 1, 1),
            },
        }
    }

    // Return the positive intersections, sorted.
    //
    pub fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let mut all_intersections = BTreeSet::new();

        for object in self.objects.iter() {
            let object_intersections = ray.intersections(object);

            if let Some((intersection_1, intersection_2)) = object_intersections {
                if intersection_1 >= 0.0 {
                    all_intersections.insert(Intersection {
                        t: intersection_1,
                        object,
                    });
                }

                if intersection_2 >= 0.0 {
                    all_intersections.insert(Intersection {
                        t: intersection_2,
                        object,
                    });
                }
            }
        }

        all_intersections.into_iter().collect::<Vec<_>>()
    }

    pub fn shade_hit(&self, intersection_state: IntersectionState) -> Color {
        intersection_state.object.material.lighting(
            &self.light_source,
            &intersection_state.point,
            &intersection_state.eyev,
            &intersection_state.normalv,
        )
    }
}
