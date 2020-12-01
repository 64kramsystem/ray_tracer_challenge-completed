use std::collections::BTreeSet;

use super::{intersection::Intersection, IntersectionState, PointLight, Ray, Shape, Sphere};
use crate::{
    lang::math::sqrt,
    lang::ApproximateFloat64Ops,
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

    // Returns the hit, and all the (sorted) intersections.
    //
    // Minor optimizations could be applied, but they're possibly not meaningful.
    //
    pub fn intersections(&self, ray: &Ray) -> (Option<Intersection>, Vec<Intersection>) {
        let mut all_intersections = BTreeSet::new();
        let mut hit: Option<Intersection> = None;

        for object in self.objects.iter() {
            let object_intersections = object.intersections(ray);

            if let Some(intersection_1) = object_intersections.0 {
                if intersection_1 >= 0.0 {
                    all_intersections.insert(Intersection {
                        t: intersection_1,
                        object: object.as_ref(),
                    });

                    // The if let version is theoretically cleaner, but in practice, it's uglier.
                    //
                    if hit.is_none() || intersection_1 < hit.as_ref().unwrap().t {
                        hit = Some(Intersection {
                            t: intersection_1,
                            object: object.as_ref(),
                        });
                    }
                }

                if let Some(intersection_2) = object_intersections.1 {
                    if intersection_2 >= 0.0 {
                        all_intersections.insert(Intersection {
                            t: intersection_2,
                            object: object.as_ref(),
                        });

                        if hit.is_none() || intersection_2 < hit.as_ref().unwrap().t {
                            hit = Some(Intersection {
                                t: intersection_2,
                                object: object.as_ref(),
                            });
                        }
                    }
                }
            }
        }

        let all_intersections = all_intersections.into_iter().collect::<Vec<_>>();

        (hit, all_intersections)
    }

    // Optimized version of intersections(), which stops at the first obstructing intersection.
    //
    pub fn is_ray_obstructed(&self, ray: &Ray, distance: f64) -> bool {
        for object in self.objects.iter() {
            let object_intersections = object.intersections(ray);

            if let Some(intersection_1) = object_intersections.0 {
                if intersection_1 >= 0.0 && intersection_1 < distance {
                    return true;
                }

                if let Some(intersection_2) = object_intersections.1 {
                    if intersection_2 >= 0.0 && intersection_2 < distance {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn shade_hit(&self, intersection_state: IntersectionState, max_recursions: u8) -> Color {
        let is_shadowed = self.is_shadowed(&intersection_state.over_point);

        let surface_color = intersection_state.object.lighting(
            &self.light_source,
            &intersection_state.point,
            &intersection_state.eyev,
            &intersection_state.normalv,
            is_shadowed,
        );

        let reflected_color = self.reflected_color(&intersection_state, max_recursions);
        let refracted_color = self.refracted_color(&intersection_state, max_recursions);

        let material = intersection_state.object.material();

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = intersection_state.schlick();

            return surface_color
                + &(reflected_color * reflectance)
                + &(refracted_color * (1.0 - reflectance));
        } else {
            return surface_color + &reflected_color + &refracted_color;
        }
    }

    pub fn color_at(&self, ray: &Ray, max_recursions: u8) -> Color {
        let (hit, intersections) = self.intersections(ray);

        if let Some(hit) = hit {
            let intersection_state = ray.intersection_state(&hit, &intersections);
            self.shade_hit(intersection_state, max_recursions)
        } else {
            COLOR_BLACK
        }
    }

    pub fn reflected_color(
        &self,
        intersection_state: &IntersectionState,
        max_recursions: u8,
    ) -> Color {
        if max_recursions == 0
            || intersection_state
                .object
                .material()
                .reflective
                .approximate()
                == 0.0
        {
            return COLOR_BLACK;
        }

        let reflect_ray = Ray {
            origin: intersection_state.over_point,
            direction: intersection_state.reflectv,
        };

        let color = self.color_at(&reflect_ray, max_recursions - 1);

        return color * intersection_state.object.material().reflective;
    }

    pub fn refracted_color(
        &self,
        intersection_state: &IntersectionState,
        max_recursions: u8,
    ) -> Color {
        if max_recursions == 0 || intersection_state.object.material().transparency == 0.0 {
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

        let cos_t = sqrt(1.0 - sin2_t);
        let direction = intersection_state.normalv * (n_ratio * cos_i - cos_t)
            - &(intersection_state.eyev * n_ratio);
        let refracted_ray = Ray {
            origin: intersection_state.under_point,
            direction,
        };

        self.color_at(&refracted_ray, max_recursions - 1)
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
