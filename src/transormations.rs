use sfml::graphics::Transform;
use sfml::system::Vector2f;

pub fn transform_from_tangent(tangent: Vector2f) -> Transform {

    let direction = Vector2f::new(1.0,0.0);

    let cos_alfa = tangent.dot(direction) / ( tangent.length_sq() * direction. length_sq()).sqrt();

    println!("alfa = {}",f32::acos(cos_alfa));

    let sin_alfa = (1.0 - cos_alfa.powi(2)).sqrt();

    let transform = Transform::new(cos_alfa,-sin_alfa,0.0,sin_alfa,cos_alfa,0.0,0.0,0.,1.0);
    return  transform;
}