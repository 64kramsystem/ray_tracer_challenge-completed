use std::collections::BTreeSet;

use super::{intersection::Intersection, IntersectionState, PointLight, Ray, Shape, Sphere};
use crate::{
    math::{Matrix, Tuple},
    properties::{Color, FlatPattern, Material, COLOR_BLACK, COLOR_WHITE},
};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light_source: PointLight,
}

impl World {
    pub fn default() -> Self {
        World {
            objects: vec![
                Box::new(Sphere {
                    material: Material {
                        pattern: Box::new(FlatPattern::new(0.8, 1.0, 0.6)),
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Material::default()
                    },
                    ..Sphere::default()
                }),
                Box::new(Sphere {
                    transform: Matrix::scaling(0.5, 0.5, 0.5),
                    ..Sphere::default()
                }),
            ],
            light_source: PointLight {
                position: Tuple::point(-10, 10, -10),
                intensity: COLOR_WHITE,
            },
        }
    }

    // Return the positive intersections, sorted.
    //
    pub fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let mut all_intersections = BTreeSet::new();

        for object in self.objects.iter() {
            let object_intersections = object.intersections(ray);

            if let Some((intersection_1, intersection_2)) = object_intersections {
                if intersection_1 >= 0.0 {
                    all_intersections.insert(Intersection {
                        t: intersection_1,
                        object: object.as_ref(),
                    });
                }

                if intersection_2 >= 0.0 {
                    all_intersections.insert(Intersection {
                        t: intersection_2,
                        object: object.as_ref(),
                    });
                }
            }
        }

        all_intersections.into_iter().collect::<Vec<_>>()
    }

    // Optimized version of intersections(), which stops at the first obstructing intersection.
    //
    pub fn is_ray_obstructed(&self, ray: &Ray, distance: f64) -> bool {
        for object in self.objects.iter() {
            let object_intersections = object.intersections(ray);

            if let Some((intersection_1, intersection_2)) = object_intersections {
                if (intersection_1 >= 0.0 && intersection_1 < distance)
                    || (intersection_2 >= 0.0 && intersection_2 < distance)
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn shade_hit(&self, intersection_state: IntersectionState) -> Color {
        let is_shadowed = self.is_shadowed(&intersection_state.over_point);

        intersection_state.object.lighting(
            &self.light_source,
            &intersection_state.point,
            &intersection_state.eyev,
            &intersection_state.normalv,
            is_shadowed,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersections(ray);

        if let Some(Intersection { t, object }) = intersections.first() {
            let intersection_state = ray.intersection_state(*t, *object);
            self.shade_hit(intersection_state)
        } else {
            COLOR_BLACK
        }
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let lightv = self.light_source.position - point;
        let distance = lightv.magnitude();
        let direction = lightv.normalize();

        let ray = Ray {
            origin: *point,
            direction,
        };

        self.is_ray_obstructed(&ray, distance)
    }
}
