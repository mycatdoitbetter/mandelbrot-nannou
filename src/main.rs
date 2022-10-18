use draw::*;

struct Complex {
    real: f32,
    imaginary: f32,
}

impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
    fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
    fn abs(&self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }
}

fn main() {
    let iterations = vec![20, 30, 75, 80, 85, 90, 100, 110, 120];

    for iteration in iterations {
        let mut canvas = Canvas::new(500, 500);


        let min_real = -2.0;
        let max_real = 1.0;
        let min_imaginary = -1.0;
        let max_imaginary = 1.0;
        
        let real_step = (max_real - min_real) / 500.0;
        let imaginary_step = (max_imaginary - min_imaginary) / 500.0;
        
        let mut real = min_real;
        

        while real < max_real {
            let mut imaginary = min_imaginary;
            while imaginary < max_imaginary {
    
                let result = belongs(real, imaginary, iteration);
    
                let x = (real - min_real) / real_step;
                let y = (imaginary - min_imaginary) / imaginary_step;
    
                if result {
                    canvas.display_list.add(
                        Drawing::new()
                        .with_shape(Shape::Circle { radius: 1 })
                        .with_xy(x, y)
                        .with_style(Style::filled(RGB::new(0, 0, 0)))
                    );
                }
    
                imaginary += imaginary_step;
            }
            real += real_step;
        }
        
        render::save(
            &canvas,
            &format!("mandelbrot-images/{}.svg", iteration),
            SvgRenderer::new(),
        )
        .expect("Failed to save");

    }
}

fn belongs(x: f32, y: f32, limit: i32) -> bool {
    let mut z = Complex {
        real: 0.0,
        imaginary: 0.0,
    };
    let c = Complex {
        real: x,
        imaginary: y,
    };

    let mut i = 0;
 

    while z.abs() < 20.0 && i < limit {
        z = z.multiply(&z).add(&c);
        i += 1;
    }
    return i == limit;
}
