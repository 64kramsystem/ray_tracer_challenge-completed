use library::Tuple;

pub struct Projectile {
    pub position: Tuple, // Point
    pub velocity: Tuple, // Vector
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            position: Tuple::point(0.0, 1.0, 0.0),
            velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
        }
    }

    // Used in/from Chapter 2.
    //
    pub fn with_values(position: Tuple, velocity: Tuple) -> Self {
        Projectile {
            position: position,
            velocity: velocity,
        }
    }

    pub fn tick(&mut self, gravity: Tuple, wind: Tuple) {
        self.position = self.position + &self.velocity;
        self.velocity = self.velocity + &gravity + &wind;
    }
}

pub struct Environment {
    pub gravity: Tuple, // Vector
    pub wind: Tuple,    // Vector

    pub projectile: Projectile,
}

impl Environment {
    pub fn new(projectile: Projectile) -> Self {
        Environment {
            gravity: Tuple::vector(0.0, -0.1, 0.0),
            wind: Tuple::vector(-0.01, 0.0, 0.0),

            projectile: projectile,
        }
    }

    pub fn tick(&mut self) {
        self.projectile.tick(self.gravity, self.wind);
    }
}

pub fn practice() {
    let projectile = Projectile::new();
    let mut environment = Environment::new(projectile);

    while environment.projectile.position.y > 0.0 {
        println!(
            "Position: x:{:.2}, y:{:.2}",
            environment.projectile.position.x, environment.projectile.position.y
        );
        environment.tick();
    }
}
