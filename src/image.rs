use std::f32::consts::PI;
use egui_sfml::egui::ViewportCommand::Transparent;
use sfml::graphics::{RenderStates, RenderTarget, RenderWindow, Sprite, Texture, Transform, Transformable};
use sfml::SfBox;
use sfml::system::Vector2f;
use crate::image::Animation::{Movement, Rotation};
use crate::transormations::{rotate_with_shear, transform_from_tangent};

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
    tangent: Vector2f,
}

impl Image {
    pub fn new(path: &str, position: Vector2f) -> Self {
        let texture = Texture::from_file(path).expect("failed to load Image");
        let transform = Transform::IDENTITY;
        Image {texture,position,animation: Animation::Movement,angle: 0.0,tangent: Vector2f::new(0.0,0.0)}
    }

    pub fn move_picture(&mut self, position: Vector2f, tangent: Vector2f) {
         self.position = position;
        self.tangent = tangent;
        //calculate transform matrix for angle

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
        sprite.set_scale(Vector2f::new(0.3,0.3));

        let rect=  sprite.global_bounds();

        let mid_point = Vector2f::new(rect.width/2.0, rect.height /2.0);
        sprite.set_position(self.position - mid_point);



        //self.transform.rotate_with_center(self.angle,self.position.x, self.position.y );
       // transform.transform_rect(&sprite);
        let mut render_states = RenderStates::default();
        let mut transform = Transform::IDENTITY;

        match self.animation {
            Rotation(RotationKind::Naive) => transform.rotate_with_center(self.angle,self.position.x,self.position.y),
            Rotation(RotationKind::WithFiltering) => transform = rotate_with_shear(self.angle,self.position),
            Movement => transform = transform_from_tangent(self.tangent,self.position)
        }

        render_states.transform = transform;
        // sprite.set_scale(Vector2f::new(0.5,0.5));
        //sprite.rotate(self.angle);
        //sprite.move(mid_point.x,mid_point.y);

        window.draw_with_renderstates(&sprite,&render_states);
    }




}