use sfml::graphics::{CircleShape, Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Shape, Transformable, Vertex};
use sfml::system::Vector2f;
use crate::bezier_curve::BezierCurve;
use crate::plain::State::Create;
const STRIP_COLOR: Color = Color::GREEN;
const POINT_COLOR: Color = Color::RED;
const CURVE_COLOR: Color = Color::BLUE;

pub enum State {
    Create,
    Edit
}

fn add_vertex(vertices: &mut Vec<Vertex>, position: Vector2f) {
    vertices.push(Vertex::with_pos_color(position,STRIP_COLOR));
}

pub fn update_node_position(vertices: &mut Vec<Vertex>,)
fn add_point(points: &mut Vec<CircleShape>, position: Vector2f) {
    let radius = 3.0;
    points.push(create_point_shape(position,radius,POINT_COLOR));
}

pub fn update_bezier(bezier_curve: &mut BezierCurve,vertices: &Vec<Vertex>) {
    bezier_curve.update_coefficients(vertices);
}

pub fn bezier_curve_update_point(bezier_curve: &mut BezierCurve,index: usize, position: Vector2f) {
    bezier_curve.update_coefficient(position,index);
}

pub fn add_node(points: &mut Vec<CircleShape>,vertices: &mut Vec<Vertex>,x:u32, y:u32) {
    let position = Vector2f::new(x as f32, y as f32);
    add_vertex(vertices,position);
    add_point(points,position);
}

pub fn render_polyline(vertices: &Vec<Vertex>, window : &mut RenderWindow) {
    window.draw_primitives(vertices,PrimitiveType::LINE_STRIP,
                           &RenderStates::default());
}

pub fn render_points(points: &Vec<CircleShape>,window: &mut RenderWindow) {
    for point in points.iter() {
        window.draw(&(point));
    }
}

pub fn create_point_shape<'a>(position: Vector2f,radius: f32, color: Color) -> CircleShape<'a> {
    let mut circle = CircleShape::default();
    circle.set_radius(radius);
    circle.set_fill_color(color);
    circle.set_position(Vector2f::new(position.x - circle.radius() / 2.0,
                                      position.y - circle.radius() / 2.0));

    circle
}