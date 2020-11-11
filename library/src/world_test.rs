use demonstrate::demonstrate;

demonstrate! {
    describe "World" {
        use crate::*;

        before {
            let world = World {
                objects: vec![
                    Sphere::from(
                        None,
                        Some(Material {
                            color: Color::new(0.8, 1.0, 0.6),
                            ambient: 0.1,
                            diffuse: 0.7,
                            specular: 0.2,
                            shininess: 200.0,
                        }),
                    ),
                    Sphere::from(
                        Some(Matrix::scaling(0.5, 0.5, 0.5)),
                        None,
                    )
                ],
                light_source: PointLight {
                    position: Tuple::point(-10, 10, -10),
                    intensity: Color::new(1, 1, 1)
                },
            };
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

        it "should shade an intersection" {
            let ray = Ray::new((0, 0, -5), (0, 0, 1));
            let sphere = &world.objects[0];
            let intersection_state = ray.intersection_state(4.0, sphere);

            let expected_shade = Color::new(0.38066, 0.47583, 0.2855);

            assert_eq!(world.shade_hit(intersection_state), expected_shade);
        }
    }
}
