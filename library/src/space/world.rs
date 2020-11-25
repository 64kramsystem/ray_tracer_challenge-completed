use std::{collections::BTreeSet, f64::NEG_INFINITY};

use super::{intersection::Intersection, IntersectionState, PointLight, Ray, Shape, Sphere};
use crate::{
    lang::NoisyFloat64,
    math::{Matrix, Tuple},
    properties::REFRACTIVE_INDEX_VACUUM,
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

    // In the book, this is part of `prepare_computations(i, r)`. This version is simpler and more intuitive.
    //
    // We select the object intersecting the ray, and divide them in two categories:
    //
    // - those "before", with i1 < t;
    // - those "after", with i2 > t.
    //
    // Then we choose, for each category, the object with the greatest i1. Done!
    //
    // Note that an object can be in both categories at the same time, when i1 < t < i2.
    //
    pub fn refraction_indexes(&self, t: f64, ray: &Ray) -> (f64, f64) {
        let (mut max_i1_before, mut max_i1_after) = (NEG_INFINITY, NEG_INFINITY);
        let (mut n_before, mut n_after) = (REFRACTIVE_INDEX_VACUUM, REFRACTIVE_INDEX_VACUUM);

        for object in self.objects.iter() {
            let object_intersections = object.intersections(ray);

            if let Some((i1, i2)) = object_intersections {
                if (i1 < t && t <= i2) && (i1 > max_i1_before) {
                    max_i1_before = i1;
                    n_before = object.material().refractive_index;
                }

                if (i1 <= t && t < i2) && (i1 > max_i1_after) {
                    max_i1_after = i1;
                    n_after = object.material().refractive_index;
                }
            }
        }

        (n_before, n_after)
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

    pub fn shade_hit(&self, intersection_state: IntersectionState, max_reflections: u8) -> Color {
        let is_shadowed = self.is_shadowed(&intersection_state.over_point);

        let surface_color = intersection_state.object.lighting(
            &self.light_source,
            &intersection_state.point,
            &intersection_state.eyev,
            &intersection_state.normalv,
            is_shadowed,
        );

        let reflected_color = self.reflected_color(intersection_state, max_reflections);

        surface_color + &reflected_color
    }

    pub fn color_at(&self, ray: &Ray, max_reflections: u8) -> Color {
        let intersections = self.intersections(ray);

        if let Some(Intersection { t, object }) = intersections.first() {
            let intersection_state = ray.intersection_state(*t, *object, &self);
            self.shade_hit(intersection_state, max_reflections)
        } else {
            COLOR_BLACK
        }
    }

    pub fn reflected_color(
        &self,
        intersection_state: IntersectionState,
        max_reflections: u8,
    ) -> Color {
        if max_reflections == 0 || intersection_state.object.material().reflective.denoise() == 0.0
        {
            return COLOR_BLACK;
        }

        let reflect_ray = Ray {
            origin: intersection_state.over_point,
            direction: intersection_state.reflectv,
        };

        let color = self.color_at(&reflect_ray, max_reflections - 1);

        return color * intersection_state.object.material().reflective;
    }

    pub fn refracted_color(
        &self,
        intersection_state: IntersectionState,
        max_refractions: u8,
    ) -> Color {
        if max_refractions == 0 || intersection_state.object.material().transparency == 0.0 {
            return COLOR_BLACK;
        }

        // Identify the case of total internal refraction, using Snell's law. See book p.157.
        //
        let n_ratio = intersection_state.n1 / intersection_state.n2;
        let cos_i = intersection_state
            .eyev
            .dot_product(&intersection_state.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return COLOR_BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = intersection_state.normalv * (n_ratio * cos_i - cos_t)
            - &(intersection_state.eyev * n_ratio);
        let refracted_ray = Ray {
            origin: intersection_state.under_point,
            direction,
        };

        self.color_at(&refracted_ray, max_refractions - 1)
            * intersection_state.object.material().transparency
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
