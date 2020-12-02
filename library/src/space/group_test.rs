#![allow(unused_imports)]

use demonstrate::demonstrate;

demonstrate! {
    describe "Group" {
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};
        use std::sync::{Arc, Mutex, Weak};

        before {
            #[allow(unused_variables)]
            let group = Group::default();
        }

        it "Creating a new group" {
            assert_eq!(*group.transform(), Matrix::identity(4));
            assert_eq!(group.children.lock().unwrap().len(), 0);
        }

        it "Adding a child to a group" {
            let shape: Arc<dyn Shape> = Arc::new(Plane::default());
            let group: Arc<dyn Shape> = Arc::new(group);

            Group::add_child(&group, &shape);

            let actual_children = group.children().lock().unwrap();

            assert_eq!(actual_children.len(), 1);
            assert_eq!(actual_children[0].id(), shape.id());

            let shape_parent_ref = &shape.parent().lock().unwrap();
            let shape_parent = Weak::upgrade(shape_parent_ref).unwrap();

            assert_eq!(shape_parent.id(), group.id());
        }

        it "Intersecting a ray with an empty group" {
            let ray = Ray::new((0, 0, 0), (0, 0, 1));

            assert_eq!(group.local_intersections(&ray).len(), 0);
        }

        it "Intersecting a ray with a nonempty group" {
            let group: Arc<dyn Shape> = Arc::new(group);

            let sphere1: Arc<dyn Shape> = Arc::new(Sphere {
                ..Sphere::default()
            });
            let sphere2: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(0, 0, -3),
                ..Sphere::default()
            });
            let sphere3: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            Group::add_child(&group, &sphere1);
            Group::add_child(&group, &sphere2);
            Group::add_child(&group, &sphere3);

            let ray = Ray::new((0, 0, -5), (0, 0, 1));

            let actual_intersections = group.local_intersections(&ray);

            assert_eq!(actual_intersections.len(), 4);

            assert_eq!(actual_intersections[0], 1.0);
            assert_eq!(actual_intersections[1], 3.0);
            assert_eq!(actual_intersections[2], 4.0);
            assert_eq!(actual_intersections[3], 6.0);
        }

        // it "Intersecting a transformed group" {
        //     Given g ← group()
        //     And set_transform(g, scaling(2, 2, 2))
        //     And s ← sphere()
        //     And set_transform(s, translation(5, 0, 0))
        //     And add_child(g, s)
        //     When r ← ray(point(10, 0, -10), vector(0, 0, 1))
        //     And xs ← intersect(g, r)
        //     Then xs.count = 2
        // }
    }
}
