use std::f32::consts::PI;
use sfml::graphics::{RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::SfBox;
use sfml::system::Vector2f;

const ROTATION_SPEED: f32 = 1.0;

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
    texture: SfBox<Texture>,
    position: Vector2f,
    pub animation: Animation,
    angle: f32,
}

impl Image {
    pub fn new(path: &str, position: Vector2f) -> Self {
        let texture = Texture::from_file(path).expect("failed to load Image");

        Image {texture,position,animation: Animation::Movement,angle: 0.0}
    }

    pub fn move_picture(&mut self, position: Vector2f, tangent: Vector2f) {
         self.position = position;
        //println!("current_angle = {}",self.angle);
    }

    pub fn rotate_picture(&mut self, angle: f32) {
        self.angle = angle % 360.0;
    }

    pub fn do_rotate_picture_frame(&mut self) {
        self.rotate_picture(self.angle + ROTATION_SPEED);
    }

    pub fn render(&self,window: &mut RenderWindow) {
        let mut sprite = Sprite::with_texture(&self.texture);
        sprite.set_scale(Vector2f::new(0.5,0.5));
        sprite.set_rotation(self.angle);
        let rect=  sprite.global_bounds();
        let mid_point = Vector2f::new(rect.width/2.0, rect.height /2.0);

        sprite.set_position(self.position - mid_point);
        window.draw(&sprite);
    }




}