use sfml::audio::listener::position;
use sfml::graphics::{CircleShape, Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Shape, Transformable, Vertex};
use sfml::system::Vector2f;
use crate::bezier_curve::BezierCurve;
use crate::image::{create_hsv_image, Image};

const STRIP_COLOR: Color = Color::BLUE;
const POINT_COLOR: Color = Color::RED;
#[derive(PartialEq,Copy, Clone)]
pub enum State {
    Create,
    Edit
}

fn add_vertex(vertices: &mut Vec<Vertex>, position: Vector2f) {
    vertices.push(Vertex::with_pos_color(position,STRIP_COLOR));
}

pub fn update_node_position(vertices: &mut Vec<Vertex>,points: &mut Vec<CircleShape>,
                            bezier_curve: &mut BezierCurve, index: usize, x: i32, y: i32) {
    let position = Vector2f::new(x as f32, y as f32);
    update_vertex_position(&mut vertices[index],position);
    update_point_position(&mut points[index],position);
    update_bezier_point(bezier_curve,index,vertices);
}

fn update_vertex_position(vertex: &mut Vertex, position: Vector2f) {
    vertex.position = position;
}

fn update_point_position(circle: &mut CircleShape, position: Vector2f) {
    circle.set_position(Vector2f::new(position.x - circle.radius() / 2.0,
                                      position.y - circle.radius() / 2.0));
}

fn add_point(points: &mut Vec<CircleShape>, position: Vector2f) {
    let radius = 3.0;
    points.push(create_point_shape(position,radius,POINT_COLOR));
}

pub fn update_bezier(bezier_curve: &mut BezierCurve,vertices: &Vec<Vertex>) {
    bezier_curve.update_coefficients(vertices);
    bezier_curve.update_curve();
    bezier_curve.update_tangent_curve();
}

fn update_bezier_point(bezier_curve: &mut BezierCurve,index: usize,vertices: &Vec<Vertex>) {
    bezier_curve.update_coefficient(vertices[index].position,index);
    bezier_curve.update_tangent_coefficient(vertices,index);
    bezier_curve.update_curve();
    bezier_curve.update_tangent_curve();
}

pub fn get_selected_point_index(vertices: &Vec<Vertex>,position: Vector2f) -> Option<usize> {
    for (index,vertex) in vertices.iter().enumerate() {
        let dist = (vertex.position - position).length_sq().sqrt();
        if dist <= (6.0 + 10.0) {return Some(index)}
    }
    return None;
}

pub fn add_node(vertices: &mut Vec<Vertex>,points: &mut Vec<CircleShape>,x:i32, y:i32) {
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
        window.draw(point);
    }
}

pub fn select_point(point_index: Option<usize>,points: &mut Vec<CircleShape>) {
    if let Some(index) = point_index {
        let radius = points[index].radius();
        let mut position = points[index].position();
        position.x = position.x - radius/ 2.0;
        position.y = position.y - radius / 2.0;
        points[index].set_radius(2.0 * radius);
        points[index].set_position(position);
    }
}

pub fn unselect_point(index: usize, points: &mut Vec<CircleShape>) {
    let radius = points[index].radius();
    points[index].set_radius(radius / 2.0);
}

fn create_point_shape<'a>(position: Vector2f,radius: f32, color: Color) -> CircleShape<'a> {
    let mut circle = CircleShape::default();
    circle.set_radius(radius);
    circle.set_fill_color(color);
    circle.set_position(Vector2f::new(position.x - circle.radius() / 2.0,
                                      position.y - circle.radius() / 2.0));

    circle
}

pub fn load_image(bezier_curve: &mut BezierCurve,path: &str) {
    bezier_curve.image = Image::new(&path,Vector2f::new(0.0,0.0));
}

pub fn create_image(bezier_curve: &mut BezierCurve) {
    bezier_curve.image = create_hsv_image();
}