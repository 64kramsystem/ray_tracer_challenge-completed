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

    pub fn tick(&mut self, gravity: Tuple, wind: Tuple) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + gravity + wind;
    }
}

pub struct Environment {
    pub gravity: Tuple, // Vector
    pub wind: Tuple,    // Vector

    pub projectile: Projectile,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            gravity: Tuple::vector(0.0, -0.1, 0.0),
            wind: Tuple::vector(-0.01, 0.0, 0.0),

            projectile: Projectile::new(),
        }
    }

    pub fn tick(&mut self) {
        self.projectile.tick(self.gravity, self.wind);
    }
}

pub fn practice() {
    let mut environment = Environment::new();

    while environment.projectile.position.y > 0.0 {
        println!(
            "Position: x:{:.2}, y:{:.2}",
            environment.projectile.position.x, environment.projectile.position.y
        );
        environment.tick();
    }
}
