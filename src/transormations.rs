use std::f32::consts::PI;
use egui_sfml::egui::ViewportCommand::Transparent;
use sfml::audio::listener::direction;
use sfml::graphics::Transform;
use sfml::system::Vector2f;

pub fn transform_from_tangent(tangent: Vector2f) -> Transform {

    let direction = Vector2f::new(1.0,0.0);

    let cos_alfa = tangent.dot(direction) / ( tangent.length_sq() * direction. length_sq()).sqrt();

    //println!("alfa = {}",f32::acos(cos_alfa));

    let sign = if tangent.dot(direction) < 0.0 {1.0} else {-1.0};


    let sin_alfa = sign *  (1.0 - cos_alfa.powi(2)).sqrt();

    let transform = Transform::new(cos_alfa,-sin_alfa,0.0,sin_alfa,cos_alfa,0.0,0.0,0.,1.0);
    return  transform;
}

pub fn naive_rotation(angle: f32, position: Vector2f) -> Transform {
    let mut transform = Transform::IDENTITY;
    transform.rotate_with_center(angle,position.x,position.y);

    transform
}

pub fn rotate_with_shear(angle: f32, position: Vector2f) -> Transform {
    let mut transform = Transform::IDENTITY;

    let mut translate = Transform::IDENTITY;
    translate.translate(position.x,position.y);
    let mut reverse_translate = Transform::IDENTITY;
    reverse_translate.translate(-position.x,-position.y);

    let radian = PI * angle / 180.0;

    let tg_alfa_half = f32::tan(radian / 2.0);
    let shear_x = Transform::new(1.0,-tg_alfa_half,0.0,0.0,1.0,0.0,0.,0.0,1.);
    let shear_y = Transform::new(1.,0.,0.,f32::sin(radian),1.0,0.,0.,0.,1.);

    transform.combine(&translate);
    transform.combine(&shear_x);
    transform.combine(&shear_y);
    transform.combine(&shear_x);
    transform.combine(&reverse_translate);

    transform
}