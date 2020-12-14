use demonstrate::demonstrate;

demonstrate! {
    describe "Group" {
        use crate::math::*;
        use crate::space::{*, shape::private::ShapeLocal};
        use std::sync::{Arc};

        it "Creating a new group" {
            let group: Arc<Group> = Arc::new(Group::default());

            assert_eq!(*group.transform(), Matrix::identity(4));
            assert_eq!((*group).children.len(), 0);
        }

        it "Adding a child to a group" {
            let shape: Arc<dyn Shape> = Arc::new(Plane::default());
            let shape_id = shape.id();

            let group = Group::new(
                Matrix::identity(4),
                vec![shape],
            );

            let actual_children = &group.children;

            assert_eq!(actual_children.len(), 1);
            assert_eq!(actual_children[0].id(), shape_id);
            assert_eq!(actual_children[0].parent().unwrap().id(), group.id());
        }

        it "Intersecting a ray with an empty group" {
            let group: Arc<Group> = Arc::new(Group::default());
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

            let sphere1_id = sphere1.id();
            let sphere2_id = sphere2.id();

            let group = Group::new(
                Matrix::identity(4),
                vec![sphere1, sphere2, sphere3],
            );

            let ray = Ray::new((0, 0, -5), (0, 0, 1));

            let actual_intersections = group.local_intersections(&ray);

            assert_eq!(actual_intersections.len(), 4);

            assert_eq!(actual_intersections[0].object.id(), sphere2_id);
            assert_eq!(actual_intersections[1].object.id(), sphere2_id);
            assert_eq!(actual_intersections[2].object.id(), sphere1_id);
            assert_eq!(actual_intersections[3].object.id(), sphere1_id);
        }

        it "Intersecting a transformed group" {
            let sphere: Arc<dyn Shape> = Arc::new(Sphere {
                transform: Matrix::translation(5, 0, 0),
                ..Sphere::default()
            });

            let group = Group::new(
                Matrix::scaling(2, 2, 2),
                vec![sphere],
            );

            let ray = Ray::new((10, 0, -10), (0, 0, 1));

            let actual_intersections = group.intersections(&ray);

            assert_eq!(actual_intersections.len(), 2);
        }
    }
}
