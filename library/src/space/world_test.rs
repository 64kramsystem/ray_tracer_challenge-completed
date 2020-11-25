use demonstrate::demonstrate;

demonstrate! {
    describe "World" {
        use crate::math::*;
        use crate::properties::*;
        use crate::space::*;

        before {
            #[allow(unused_variables,unused_mut)]
            let mut world = World::default();

            #[allow(non_snake_case,unused_variables)]
            let SQRT_TWO = 2.0_f64.sqrt();
        }

        it "should intersect with a ray" {
            let ray = Ray::new((0, 0, -5), (0, 0, 1));

            let intersections = world
                .intersections(&ray)
                .iter()
                .map(|intersection| intersection.t).collect::<Vec<_>>();

            let expected_intersections = vec![4.0, 4.5, 5.5, 6.0];

            assert_eq!(intersections, expected_intersections);
        }

        it "should find the refractive indexes at various scenarios" {
            let examples = [
                // [t, n1, n2]
                //
                [2.0,  1.0, 1.5],
                [2.75, 1.5, 2.0],
                [3.25, 2.0, 2.5],
                [4.75, 2.5, 2.5],
                [5.25, 2.5, 1.5],
                [6.00, 1.5, 1.0],
            ];

            let sphere1 = Sphere {
                transform: Matrix::scaling(2, 2, 2),
                material: Material {
                    refractive_index: 1.5,
                    ..Material::default()
                },
                ..Sphere::default()
            };
            let sphere2 = Sphere {
                transform: Matrix::translation(0.0, 0.0, -0.25),
                material: Material {
                    refractive_index: 2.0,
                    ..Material::default()
                },
                ..Sphere::default()
            };
            let sphere3 = Sphere {
                transform: Matrix::translation(0.0, 0.0, 0.25),
                material: Material {
                    refractive_index: 2.5,
                    ..Material::default()
                },
                ..Sphere::default()
            };

            let objects: Vec<Box<dyn Shape>> = vec![
                Box::new(sphere1),
                Box::new(sphere2),
                Box::new(sphere3),
            ];

            let world = World {
                objects,
                ..World::default()
            };

            let ray = Ray::new((0, 0, -4), (0, 0, 1));

            for (i, [t, expected_n1, expected_n2]) in examples.iter().enumerate() {
                let (actual_n1, actual_n2) = world.refraction_indexes(*t, &ray);

                assert_eq!(actual_n1, *expected_n1, "Example {}: E:{}/{} A:{}/{}", i, expected_n1, expected_n2, actual_n1, actual_n2);
                assert_eq!(actual_n2, *expected_n2, "Example {}: E:{}/{} A:{}/{}", i, expected_n1, expected_n2, actual_n1, actual_n2);
            }
        }

        context "intersection shading" {
            it "should be performed in direct light" {
                let ray = Ray::new((0, 0, -5), (0, 0, 1));
                let sphere = &world.objects[0];
                let intersection_state = ray.intersection_state(4.0, sphere.as_ref(), &world);

                let expected_shade = Color::new(0.38066, 0.47583, 0.2855);

                assert_eq!(world.shade_hit(intersection_state, 0), expected_shade);
            }

            it "should be performed in the shadow" {
                let sphere1 = Sphere::default();
                let sphere2a = Sphere {
                    transform: Matrix::translation(0, 0, 10),
                    ..Sphere::default()
                };
                let sphere2b = Sphere {
                    transform: Matrix::translation(0, 0, 10),
                    ..Sphere::default()
                };

                let objects: Vec<Box<dyn Shape>> = vec![Box::new(sphere1), Box::new(sphere2a)];

                let light_source = PointLight::new(
                    (0, 0, -10),
                    (1, 1, 1),
                );

                let world = World { objects, light_source };

                let ray = Ray::new(
                    (0, 0, 5),
                    (0, 0, 1),
                );

                let intersection_state = ray.intersection_state(4.0, &sphere2b, &world);

                let expected_color = Color::new(0.1, 0.1, 0.1);

                assert_eq!(world.shade_hit(intersection_state, 0), expected_color);
            }

            it "should be performed with a reflective material" {
                let plane = Plane {
                    material: Material {
                        reflective: 0.5,
                        ..Material::default()
                    },
                    transform: Matrix::translation(0, -1, 0),
                    ..Plane::default()
                };

                world.objects.push(Box::new(plane));

                let ray = Ray::new((0, 0, -3), (0.0, -SQRT_TWO / 2.0, SQRT_TWO / 2.0));

                let plane_ref = world.objects.last().unwrap().as_ref();
                let intersection_state = ray.intersection_state(SQRT_TWO, plane_ref, &world);

                let actual_color = world.shade_hit(intersection_state, 1);

                assert_eq!(actual_color, Color::new(0.87677, 0.92436, 0.82918));
            }

            it "should be performed with mutually reflective surfaces" {
                world.light_source = PointLight::new((0, 0, 0), (1, 1, 1));

                let lower_plane = Plane {
                    material: Material {
                        reflective: 1.0,
                        ..Material::default()
                    },
                    transform: Matrix::translation(0, -1, 0),
                    ..Plane::default()
                };

                world.objects.push(Box::new(lower_plane));

                let upper_plane = Plane {
                    material: Material {
                        reflective: 1.0,
                        ..Material::default()
                    },
                    transform: Matrix::translation(0, 1, 0),
                    ..Plane::default()
                };

                world.objects.push(Box::new(upper_plane));

                let ray = Ray::new((0, 0, 0), (0, 1, 0));

                world.color_at(&ray, 5);
            }
        } // context "intersection shading"

        context "color of a ray intersection" {
            it "when a ray misses" {
                let ray =  Ray::new((0, 0, -5), (0, 1, 0));
                let expected_color = COLOR_BLACK;

                assert_eq!(world.color_at(&ray, 0), expected_color);
            }

            it "when a ray hits" {
                let ray = Ray::new((0, 0, -5), (0, 0, 1));
                let expected_color = Color::new(0.38066, 0.47583, 0.2855);

                assert_eq!(world.color_at(&ray, 0), expected_color);
            }

            it "with the intersection behind the ray" {
                let ray = Ray::new((0.0, 0.0, 0.75), (0, 0, -1));

                // Differs in the material ambient value.
                //
                let world = World {
                    objects: vec![
                        Box::new(Sphere {
                            material: Material {
                                pattern: Box::new(FlatPattern::new(0.8, 1.0, 0.6)),
                                ambient: 1.0,
                                diffuse: 0.7,
                                specular: 0.2,
                                ..Material::default()
                            },
                            ..Sphere::default()
                        }),
                        Box::new(Sphere {
                            material: Material {
                                ambient: 1.0,
                                ..Material::default()
                            },
                            transform: Matrix::scaling(0.5, 0.5, 0.5),
                            ..Sphere::default()
                        }),
                    ],
                    ..World::default()
                };

                // With the flat pattern, the color is the same at any point.
                //
                let expected_color = world.objects[1].material().pattern.color_at(&Tuple::point(0, 0, 0));

                assert_eq!(world.color_at(&ray, 0), expected_color);
            }
        } // context "color of a ray intersection"

        context "reflected color" {
            it "should be computed for a nonreflective material" {
                let ray = Ray::new((0, 0, 0), (0, 0, 1));
                let shape = Sphere {
                    transform: Matrix::scaling(0.5, 0.5, 0.5),
                    material: Material {
                        ambient: 1.0,
                        ..Material::default()
                    },
                    ..Sphere::default()
                };

                let intersection_state = ray.intersection_state(1.0, &shape, &world);

                let actual_color = world.reflected_color(intersection_state, 0);

                assert_eq!(actual_color, COLOR_BLACK);

            }

            it "should be computed for a reflective material" {
                let plane = Plane {
                    material: Material {
                        reflective: 0.5,
                        ..Material::default()
                    },
                    transform: Matrix::translation(0, -1, 0),
                    ..Plane::default()
                };

                world.objects.push(Box::new(plane));

                let ray = Ray::new((0, 0, -3), (0.0, -SQRT_TWO / 2.0, SQRT_TWO / 2.0));

                let plane_ref = world.objects.last().unwrap().as_ref();
                let intersection_state = ray.intersection_state(SQRT_TWO, plane_ref, &world);

                let actual_color = world.reflected_color(intersection_state, 1);

                assert_eq!(actual_color, Color::new(0.19032, 0.2379, 0.14274));
            }
        } // context "reflected color"

        context "refracted color" {
            it "should be computed for an opaque material" {
                let ray = Ray::new((0, 0, -5), (0, 0, 1));
                let intersection_state = ray.intersection_state(4.0, world.objects[0].as_ref(), &world);
                let expected_color = COLOR_BLACK;

                assert_eq!(world.refracted_color(intersection_state, 5), expected_color);
            }

            it "should be computed for a refractive material, at the maximum recursion depth" {
                let ray = Ray::new((0, 0, -5), (0, 0, 1));

                let new_shape = Sphere {
                    material: Material {
                        pattern: Box::new(FlatPattern::new(0.8, 1.0, 0.6)),
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                        transparency: 1.0,
                        refractive_index: 1.5,
                        ..Material::default()
                    },
                    ..Sphere::default()
                };

                world.objects[0] = Box::new(new_shape);

                let intersection_state = ray.intersection_state(4.0, world.objects[0].as_ref(), &world);
                let expected_color = COLOR_BLACK;

                assert_eq!(world.refracted_color(intersection_state, 0), expected_color);
            }

            it "should return black in case of total internal refraction" {
                let new_shape = Sphere {
                    material: Material {
                        pattern: Box::new(FlatPattern::new(0.8, 1.0, 0.6)),
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                        transparency: 1.0,
                        refractive_index: 1.5,
                        ..Material::default()
                    },
                    ..Sphere::default()
                };

                let ray = Ray::new((0.0, 0.0, SQRT_TWO / 2.0), (0, 1, 1));
                world.objects[0] = Box::new(new_shape);

                // We're taking the intersection from inside the sphere.
                //
                let intersection_state = ray.intersection_state(5.0, world.objects[0].as_ref(), &world);
                let expected_color = COLOR_BLACK;

                assert_eq!(world.refracted_color(intersection_state, 0), expected_color);
            }
        } // context "refracted color"

        context "shadowing" {
            it "should find when a point is not in the shadow" {
                let point = Tuple::point(10, -10, 10);

                assert!(world.is_shadowed(&point));
            }

            it "should find when a point is in the shadow" {
                let point = Tuple::point(-20, 20, -20);

                assert!(!world.is_shadowed(&point));
            }
        } // context "shadowing"
    }
}
