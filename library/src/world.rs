use std::collections::BTreeSet;

use crate::{
    intersection::Intersection, Color, IntersectionState, Material, Matrix, PointLight, Ray,
    Sphere, Tuple,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: PointLight,
}

impl World {
    pub fn default() -> Self {
        World {
            objects: vec![
                Sphere {
                    material: Material {
                        color: Color::new(0.8, 1.0, 0.6),
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                        shininess: 200.0,
                    },
                    ..Sphere::default()
                },
                Sphere {
                    transformation: Matrix::scaling(0.5, 0.5, 0.5),
                    ..Sphere::default()
                },
            ],
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

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersections(ray);

        if let Some(Intersection { t, object }) = intersections.first() {
            let intersection_state = ray.intersection_state(*t, object);
            self.shade_hit(intersection_state)
        } else {
            Color::new(0, 0, 0)
        }
    }
}
