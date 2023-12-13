use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::system::{Vector2f};
use crate::image;
use crate::image::{Image, RotationKind};

const DISCREET_POINTS: usize = 1_000;
pub struct BezierCurve {
    pub curve : Vec<Vertex>,
    pub tangent_curve: Vec<Vector2f>,
    pub x_coefficients: Vec<f32>,
    pub y_coefficients: Vec<f32>,
    pub x_tangent_coefficients: Vec<f32>,
    pub y_tangent_coefficients: Vec<f32>,
    pub image: Image,
    pub time_index: usize,
}

impl BezierCurve {
    pub fn new() -> Self {
        let image = Image::new("Data/image.png",Vector2f::new(-999.0,-999.0));
        let time_index = 0;
        BezierCurve{curve: vec![], tangent_curve: vec![],x_coefficients:vec![],y_coefficients:vec![],
            x_tangent_coefficients: vec![], y_tangent_coefficients: vec![],image,time_index}
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
        //self.image.move_picture(self.curve[0].position,Vector2f::new(0.0,0.0));
    }

    pub fn update_tangent_curve(&mut self) {
        self.tangent_curve = vec![Vector2f::new(0.0,0.0);DISCREET_POINTS];
        let dt = 1.0 / DISCREET_POINTS as f32;
        let n = self.x_tangent_coefficients.len();
        let mut t_point: f32 = 0.0;
        for i in 0..DISCREET_POINTS {
            let mut x_coord = 0.0;
            let mut y_coord = 0.0;
            for j in 0..n {
                let t_val = (1.0 - t_point).powi((n - 1) as i32 - j as i32) *
                    t_point.powi(j as i32);

                x_coord = x_coord + t_val * self.x_tangent_coefficients[j];
                y_coord = y_coord + t_val * self.y_tangent_coefficients[j];

            }
            self.tangent_curve[i] = Vector2f::new((n+1) as f32* x_coord, (n+1) as f32 * y_coord);
            //println!("tangent point {}, {}",x_coord,y_coord);
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

    fn update_tangent_coefficients_with_binom(&mut self,vertices: &Vec<Vertex>,index:usize, factorials: &Vec<f32>) {
        let n = self.y_tangent_coefficients.len();

        let binom = factorials[n-1] / (factorials[n -1 - index] * factorials[index]);
        self.x_tangent_coefficients[index] = binom * (vertices[index + 1].position.x - vertices[index].position.x);
        self.y_tangent_coefficients[index] = binom * (vertices[index + 1].position.y - vertices[index].position.y);
    }

    pub fn update_tangent_coefficient(&mut self, vertices: &Vec<Vertex>,index: usize) {
        let n  = self.y_tangent_coefficients.len();
        let factorials = calculate_factorials(n);
        if index > 0 {self.update_tangent_coefficients_with_binom(vertices,index - 1,&factorials) }
        if index < n -1 {self.update_tangent_coefficients_with_binom(vertices,index,&factorials)}
    }

    pub fn update_coefficients(&mut self, vertices: &Vec<Vertex>) {
        let n = vertices.len();
        self.x_coefficients = vec![0.0;n];
        self.y_coefficients = vec![0.0;n];

        self.y_tangent_coefficients = vec![0.0; n-1];
        self.x_tangent_coefficients = vec![0.0; n-1];

        let factorials = calculate_factorials(n);

        for i in 0..n {
            let binom = factorials[n-1] / (factorials[n -1 - i] * factorials[i]);
            self.x_coefficients[i] = binom * vertices[i].position.x;
            self.y_coefficients[i] = binom * vertices[i].position.y;
        }

        for i in 0..n-1 {
            let binom = factorials[n-1 -1] / (factorials[n-1 -1 - i] * factorials[i]);
            self.x_tangent_coefficients[i] = binom * (vertices[i + 1].position.x - vertices[i].position.x);
            self.y_tangent_coefficients[i] = binom * (vertices[i + 1].position.y - vertices[i].position.y);
        }
    }

    pub fn move_image(&mut self) {
        self.time_index  = (self.time_index + 1 ) % self.curve.len();
        self.image.move_picture(self.curve[self.time_index].position,self.tangent_curve[self.time_index]);
    }

    pub fn render(&self, window: &mut RenderWindow) {
        window.draw_primitives(&self.curve,PrimitiveType::POINTS,&RenderStates::default());
        self.image.render(window);
    }

    fn rotate_naive(&mut self) {
        self.image.do_rotate_picture_frame();
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
                self.rotate_naive();
            }
        }
    }


    pub fn clear(&mut self) {
        self.curve.clear();
        self.x_coefficients.clear();
        self.y_coefficients.clear();
        self.tangent_curve.clear();
        self.y_tangent_coefficients.clear();
        self.x_tangent_coefficients.clear();
    }
}


fn calculate_factorials(n: usize) -> Vec<f32> {
    let mut factorials:Vec<f32> = vec![1.0;n];
    for i in 1..n {
        factorials[i] = factorials[i - 1] * i as f32;
    }
    factorials
}
