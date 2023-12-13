use egui_sfml::egui::Area;
use hsv::hsv_to_rgb;
use sfml::graphics::{IntRect, RenderStates, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::{graphics, SfBox};
use sfml::system::Vector2f;
use crate::image::Animation::{Movement, Rotation};
use crate::transormations::{naive_rotation, rotate_with_shear, transform_from_tangent};

const ROTATION_SPEED: f32 = 2.0;
const HEIGHT: usize = 500;
const WIDTH: usize = 500;

const IMAGE_SIZE: usize = 500;
const CENTER_X: f32 = IMAGE_SIZE as f32 / 2.0;
const CENTER_Y: f32 = IMAGE_SIZE as f32 / 2.0;
const RADIUS: f32 = IMAGE_SIZE as f32 / 3.0 ;

#[derive(PartialEq,Copy, Clone)]

pub enum RotationKind {
    Naive,
    WithFiltering,
}
#[derive(PartialEq,Copy, Clone)]

pub enum Animation {
    Rotation(RotationKind),
    Movement,
}



pub struct Image {
    pub path: String,
    texture: SfBox<Texture>,
    position: Vector2f,
    pub animation: Animation,
    angle: f32,
    tangent: Vector2f,
}

impl Image {
    pub fn new(path: &str, position: Vector2f) -> Self {
        let texture = Texture::from_file(path).expect("failed to load Image");
        Image {path: path.to_string(),texture,position,animation: Movement,angle: 0.0,tangent: Vector2f::new(0.0,0.0)}
    }

    pub fn move_picture(&mut self, position: Vector2f, tangent: Vector2f) {
         self.position = position;
        self.tangent = tangent;
    }

    pub fn rotate_picture(&mut self, angle: f32) {
        self.angle = angle % 360.0;
    }

    pub fn do_rotate_picture_frame(&mut self) {
        self.rotate_picture(self.angle + ROTATION_SPEED);
    }

    pub fn render(&self,window: &mut RenderWindow) {
        let mut sprite = Sprite::with_texture(&self.texture);
        sprite.set_scale(Vector2f::new(0.3,0.3));

        let rect=  sprite.global_bounds();

        let mid_point = Vector2f::new(rect.width/2.0, rect.height /2.0);
        sprite.set_position(self.position - mid_point);

        let mut render_states = RenderStates::default();

        let transform = match self.animation {
            Rotation(RotationKind::Naive) =>  naive_rotation(self.angle,self.position),
            Rotation(RotationKind::WithFiltering) => rotate_with_shear(self.angle,self.position),
            Movement =>  transform_from_tangent(self.tangent,self.position)
        };

        render_states.transform = transform;

        window.draw_with_renderstates(&sprite,&render_states);
    }
}

pub fn create_hsv_image() -> Image {

    let mut texture = Texture::new().unwrap();
    let _ = texture.create(WIDTH as u32, HEIGHT as u32);

    //create pixels
   // let mut pixel: [u8; 4 * WIDTH as usize * HEIGHT as usize] = [0;4*WIDTH*HEIGHT];

    //generate white background

    let pixels = create_hsv_circle();



    unsafe {
        texture.update_from_pixels(&pixels, WIDTH as u32, HEIGHT as u32,0,0);
    }

   // let mut texture = Texture::create_from_pixels(WIDTH, HEIGHT, &pixel).expect("Failed to create texture");


    //let area: IntRect

  //  let texture = Texture::load_from_image()


    Image {path: "jeden".to_string(),texture,position: Vector2f::new(0.0,0.0),animation: Movement,angle: 0.0,tangent: Vector2f::new(0.0,0.0)}
}

fn create_hsv_circle() -> [u8;WIDTH * HEIGHT * 4] {
    let mut pixels: [u8; 4 * WIDTH as usize * HEIGHT as usize] = [255;4*WIDTH*HEIGHT];
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let dx = x as f32 - CENTER_X;
            let dy = y as f32 - CENTER_Y;

            let index = ((y * IMAGE_SIZE + x) * 4) as usize;


            if dx.abs()  > (IMAGE_SIZE /2 - 50) as f32 || dy.abs() > (IMAGE_SIZE /2 - 50) as f32 {
                pixels[index] = 0;
                pixels[index + 1] = 0;
                pixels[index + 2] = 0;
                pixels[index + 3] = 255;
                continue;
            }

            let distance = (dx.powi(2) + dy.powi(2)).sqrt();

            if distance > RADIUS {continue}

            // polar coordinates
            let radius = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);

            let hue = angle.to_degrees() + 180.0;
            let saturation = f32::min(1.0, radius / RADIUS);
            let value = 1.0;

            let rgb = hsv_to_rgb(hue as f64, saturation as f64 ,value);

            pixels[index] = rgb.0;
            pixels[index + 1] = rgb.1;
            pixels[index + 2] = rgb.2;
            pixels[index + 3] = 255;
        }
    }

    pixels
}