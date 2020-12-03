#![allow(unused_imports)]

use demonstrate::demonstrate;

demonstrate! {
    describe "Group" {
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};
        use std::sync::{Arc, Mutex, Weak};

        before {
            #[allow(unused_variables)]
            let group: Arc<dyn Shape> = Arc::new(Group::default());
        }

        it "Creating a new group" {
            assert_eq!(*group.transform(), Matrix::identity(4));
            assert_eq!((*group).children().len(), 0);
        }

        it "Adding a child to a group" {
            let shape: Arc<dyn Shape> = Arc::new(Plane::default());

            Group::add_child(&group, &shape);

            let actual_children = group.children();

            assert_eq!(actual_children.len(), 1);
            assert_eq!(actual_children[0].id(), shape.id());

            assert_eq!(shape.parent().unwrap().id(), group.id());
        }

        it "Intersecting a ray with an empty group" {
            let ray = Ray::new((0, 0, 0), (0, 0, 1));

            assert_eq!(group.local_intersections(&ray).len(), 0);
        }

        it "Intersecting a ray with a nonempty group" {
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

        it "Intersecting a transformed group" {
            let group: Arc<dyn Shape> = Arc::new(Group {
                transform: Matrix::scaling(2, 2, 2),
                ..Group::default()
            });

            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            Group::add_child(&group, &sphere);

            let ray = Ray::new((10, 0, -10), (0, 0, 1));

            let actual_intersections = group.intersections(&ray);

            assert_eq!(actual_intersections.len(), 2);
        }
    }
}
