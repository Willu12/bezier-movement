use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::system::{Vector2f};
use crate::image;
use crate::image::{Image, RotationKind};

const DISCREET_POINTS: usize = 1_000;
pub struct BezierCurve {
    pub curve : Vec<Vertex>,
    pub tanget_curve: Vec<Vector2f>,
    pub x_coefficients: Vec<f32>,
    pub y_coefficients: Vec<f32>,
    pub image: Image,
    pub time_index: usize,
}

impl BezierCurve {
    pub fn new() -> Self {
        let image = Image::new("Data/jeden.png",Vector2f::new(-999.0,-999.0));
        let time_index = 0;
        BezierCurve{curve: vec![], tanget_curve: vec![],x_coefficients:vec![],y_coefficients:vec![],image,time_index}
    }

    pub fn update_curve(&mut self) {
        self.curve = vec![Vertex::with_pos_color(Vector2f::new(0.0,0.0),
                                                 Color::MAGENTA); DISCREET_POINTS];

        let dt = 1.0 / DISCREET_POINTS as f32;
        let n = self.x_coefficients.len();
        let mut t_point: f32 = 0.0;
        for i in 0..DISCREET_POINTS {
            let mut x_coord = 0.0;
            let mut y_coord = 0.0;
            for j in 0..n {
                let t_val = (1.0 - t_point).powi((n - 1) as i32 - j as i32) *
                    t_point.powi(j as i32);

                x_coord = x_coord + t_val * self.x_coefficients[j];
                y_coord = y_coord + t_val * self.y_coefficients[j];
            }
            self.curve[i].position = Vector2f::new(x_coord,y_coord);
            t_point = t_point + dt;
        }
        self.image.move_picture(self.curve[0].position,Vector2f::new(0.0,0.0));
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

    pub fn move_image(&mut self) {
        self.time_index  = (self.time_index + 1 ) % self.curve.len();
        println!("current time index = {} ",self.time_index);
        self.image.move_picture(self.curve[self.time_index].position,Vector2f::default());
    }

    pub fn render(&self, window: &mut RenderWindow) {
        window.draw_primitives(&self.curve,PrimitiveType::POINTS,&RenderStates::default());
        self.image.render(window);
    }

    fn rotate_naive(&mut self) {
        self.image.do_rotate_picture_frame();
    }

    fn rotate_with_filter(&mut self) {

    }

    pub fn do_frame(&mut self) {

        match self.image.animation {
            image::Animation::Movement => {
                self.move_image();
            }
            image::Animation::Rotation(RotationKind::Naive) => {
                self.rotate_naive();
            }
            image::Animation::Rotation(RotationKind::WithFiltering) => {
                self.rotate_with_filter();
            }
        }
    }


    pub fn clear(&mut self) {
        self.curve.clear();
        self.x_coefficients.clear();
        self.y_coefficients.clear();
        self.tanget_curve.clear();
    }
}


fn calculate_factorials(n: usize) -> Vec<f32> {
    let mut factorials:Vec<f32> = vec![1.0;n];
    for i in 1..n {
        factorials[i] = factorials[i - 1] * i as f32;
    }
    factorials
}
