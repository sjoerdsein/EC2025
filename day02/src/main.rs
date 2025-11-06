use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
struct Complex {
    x: i64,
    y: i64,
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl ops::MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl ops::DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

fn part1(a: Complex) {
    let mut res = Complex { x: 0, y: 0 };

    for _ in 0..3 {
        res *= res;
        res /= Complex { x: 10, y: 10 };
        res += a;
    }

    println!("{}", res)
}

fn engraving_depth(point: Complex, max_depth: i32) -> i32 {
    let mut res = Complex { x: 0, y: 0 };
    for c in 0..max_depth {
        res *= res;
        res /= Complex {
            x: 100_000,
            y: 100_000,
        };
        res += point;

        if std::cmp::max(res.x.abs(), res.y.abs()) > 1_000_000 {
            return c;
        }
    }

    max_depth
}

fn to_engrave(point: Complex) -> bool {
    engraving_depth(point, 100) >= 100
}

fn part2(topleft: Complex) {
    let mut number = 0;
    for y in 0..101 {
        for x in 0..101 {
            number += to_engrave(Complex {
                x: topleft.x + x * 10,
                y: topleft.y + y * 10,
            }) as i32;
        }
    }

    println!("{}", number)
}

fn part3(topleft: Complex) {
    let mut number = 0;
    for y in 0..1001 {
        for x in 0..1001 {
            number += to_engrave(Complex {
                x: topleft.x + x,
                y: topleft.y + y,
            }) as i32;
        }
    }

    println!("{}", number)
}

#[cfg(feature = "save-image")]
fn part3_image(topleft: Complex, filepath: &str) {
    let mut fractal = image::GrayImage::new(1001, 1001);

    for y in 0..1001 {
        for x in 0..1001 {
            let pixelvalue = engraving_depth(
                Complex {
                    x: topleft.x + x,
                    y: topleft.y + y,
                },
                255,
            ) as u8;
            fractal.put_pixel(x as u32, y as u32, image::Luma([pixelvalue]));
        }
    }

    fractal.save(filepath).expect("Failed to write image");
}

fn main() {
    part1(Complex { x: 156, y: 57 });
    part2(Complex {
        x: -79705,
        y: 15616,
    });
    part3(Complex {
        x: -79705,
        y: 15616,
    });

    #[cfg(feature = "save-image")]
    part3_image(
        Complex {
            x: -79705,
            y: 15616,
        },
        "fractal.png",
    );
}
