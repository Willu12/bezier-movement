use sfml::graphics::{CircleShape, Vertex};
use crate::bezier_curve::BezierCurve;
use crate::plain::{add_node, State};

pub fn mouse_click_handler(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                           bezier_curve: &mut BezierCurve,state: &State, x: i32, y:i32) {
    match state {
        State::Create => {
            add_node(vertices,points,x,y);
        }
        State::Edit => {

        }
    }
}