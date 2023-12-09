use sfml::graphics::{CircleShape, Vertex};
use crate::bezier_curve::BezierCurve;
use crate::plain::{add_node, State};

pub fn mouse_click_handler(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                           bezier_curve: &mut BezierCurve,state: State, x: i32, y:i32) {
    match state {
        State::Create => {
            add_node(vertices,points,x,y);
        }
        State::Edit => {

        }
    }
}

pub fn start_creating_new_curve(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                                bezier_curve: &mut BezierCurve,state: State) -> State {
    if state == State::Create {return state}
    vertices.clear();
    points.clear();
    bezier_curve.clear();

    State::Create
}

pub fn build_new_curve(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                       bezier_curve: &mut BezierCurve,state: State) -> State {
    if state == State::Edit {return state}

    points.remove(points.len() - 1);
    vertices.remove(vertices.len() - 1);
    bezier_curve.update_coefficients(vertices);
    bezier_curve.update_curve();

    State::Edit
}