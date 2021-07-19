#[derive(Default, Debug)]

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, point2: &Point) -> f64 {
        let delta_x = point2.x - self.x;
        let delta_y = point2.y - self.y;

        (delta_x * delta_x + delta_y * delta_y).sqrt()
    }
}

#[derive(Debug)]
struct Projectile {
    points: Vec<Point>,
    angle: f64,
    maxY: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Projectile {
    fn setup(angle: f64, speed: f64) -> Self {
        Self {
            points: Vec::from(vec![Point::default()]),
            angle,
            maxY: 0.0,
            vx: angle.to_radians().cos() * speed,
            vy: angle.to_radians().sin() * speed,
            x: 0.0,
            y: 0.0,
        }
    }
    fn update(&mut self, time: f64) {
        self.x = self.vx * time;
        self.y = self.vy * time + (0.5) * -9.81 * time * time;

        if self.y > self.maxY {
            self.maxY = self.y
        }
        if self.y > 0.0 {
            self.points.push(Point {
                x: self.x,
                y: self.y,
            });
        }
    }
}

pub fn normalize<T, I: 'static>(value: T, from_min: I, from_max: I, to_min: I, to_max: I) -> I
where
    T: num::cast::AsPrimitive<I>,
    I: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

fn main() {
    let start = 0.0;
    let end = 90.0;
    let number_of_projectiles = 90;
    let number_of_iterations = 90;
    let timespan = 25.0; // Seconds

    // Create the projectile
    let mut projectiles: Vec<Projectile> = Vec::new();
    for x in 0..number_of_projectiles {
        let angle = normalize(x, 0.0, number_of_projectiles.into(), start, end);
        projectiles.push(Projectile::setup(angle, 100.0));
    }

    // Generate all the paths of the projectiles
    for t in 0..25000 {
        let time = normalize(t, 0.0, number_of_iterations.into(), 0.0, timespan);
        projectiles.iter_mut().for_each(|p| p.update(time));
    }

    let distances: Vec<f64> = projectiles
        .iter()
        .map(|p| {
            let mut distance = 0.0;
            for i in 0..(p.points.len() - 1) {
                distance += p.points[i].distance(&p.points[i + 1])
            }
            distance
        })
        .collect();

    //     { points, angle, maxY, x, y, vx, vy }
}
