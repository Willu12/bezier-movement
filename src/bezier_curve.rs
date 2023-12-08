use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::system::{Vector2f};

const DISCREET_POINTS: usize = 1_000_000;
pub struct BezierCurve {
    pub curve : Vec<Vertex>,
    pub x_coefficients: Vec<f32>,
    pub y_coefficients: Vec<f32>,
}

impl BezierCurve {
    pub fn new() -> Self {
        BezierCurve{curve: vec![], x_coefficients:vec![],y_coefficients:vec![]}
    }

    pub fn update_curve(&mut self) {
        self.curve = vec![Vertex::with_pos_color(Vector2f::new(0.0,0.0),
                                                 Color::BLUE); DISCREET_POINTS];

        let dt = 1.0 / DISCREET_POINTS as f32;
        let n = self.x_coefficients.len();
        let mut t_point: f32 = 0.0;
        for i in 0..DISCREET_POINTS {
            let mut x_coord = 0.0;
            let mut y_coord = 0.0;
            for j in 0..n {
                let t_val = (1.0 - t_point).powi(n as i32 - j as i32) *
                    t_point.powi(j as i32);

                x_coord = x_coord + t_val * self.x_coefficients[j];
                y_coord = y_coord + t_val * self.y_coefficients[j];
            }
            self.curve[i].position = Vector2f::new(x_coord,y_coord);
            t_point = t_point + dt;
        }
    }

    pub fn update_coefficient(&mut self, position: Vector2f, index: usize) {
        let n = self.y_coefficients.len();
        let factorials = calculate_factorials(n);
        let binom = factorials[n-1] / (factorials[n -1 - index] * factorials[index]);
        self.x_coefficients[index] = binom * position.x;
        self.y_coefficients[index] = binom * position.y;

    }

    pub fn update_coefficients(&mut self, vertices: &Vec<Vertex>) {
        let n = vertices.len();
        self.x_coefficients = vec![0.0;n];
        self.y_coefficients = vec![0.0;n];

        let factorials = calculate_factorials(n);

        for i in 0..n {
            let binom = factorials[n-1] / (factorials[n -1 - i] * factorials[i]);
            self.x_coefficients[i] = binom * vertices[i].position.x;
            self.y_coefficients[i] = binom * vertices[i].position.y;
        }
    }

    pub fn render(&self, window: &mut RenderWindow) {
        window.draw_primitives(&self.curve,PrimitiveType::POINTS,&RenderStates::default());
    }
}


fn calculate_factorials(n: usize) -> Vec<f32> {
    let mut factorials:Vec<f32> = vec![1.0;n];
    for i in 1..n {
        factorials[i] = factorials[i - 1] * i as f32;
    }
    factorials
}
