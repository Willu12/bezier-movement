use std::f32::consts::PI;
use sfml::graphics::Transform;
use sfml::system::Vector2f;

pub fn rotate_from_tangent(tangent: Vector2f) -> Transform {
    let normalized_tangent = tangent / (tangent.dot(tangent)).sqrt();
    let perpendicular = Vector2f::new(-normalized_tangent.y,normalized_tangent.x);
    let transform = Transform::new(normalized_tangent.x,perpendicular.x,0.,normalized_tangent.y,perpendicular.y,0.,0.,0.,1.);
    return  transform;
}

pub fn transform_from_tangent(tangent:Vector2f, position: Vector2f) -> Transform {

    let mut transformation = Transform::IDENTITY;
    transformation.translate(position.x,position.y);
    let rotate = rotate_from_tangent(tangent);
    transformation.combine(&rotate);
    transformation.translate(-position.x,-position.y);

    transformation
}

pub fn naive_rotation(angle: f32, position: Vector2f) -> Transform {
    let mut transform = Transform::IDENTITY;
    transform.rotate_with_center(angle,position.x,position.y);

    transform
}

pub fn rotate_with_shear(mut angle: f32, position: Vector2f) -> Transform {
    let mut transform = Transform::IDENTITY;
    
    let mut translate = Transform::IDENTITY;
    translate.translate(position.x,position.y);
    let mut reverse_translate = Transform::IDENTITY;
    reverse_translate.translate(-position.x,-position.y);

    if (angle/2.0 + 90.0) % 180.0 == 0.0 {angle += 1.0} //ensure tg_alfa_half exists
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
