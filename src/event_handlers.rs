use sfml::graphics::{CircleShape, Shape, Vertex};
use sfml::system::Vector2f;
use crate::bezier_curve::BezierCurve;
use crate::image::Image;
use crate::plain::{add_node, get_selected_point_index, select_point, State, update_node_position};

pub fn mouse_click_handler(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                           bezier_curve: &mut BezierCurve,mut selected_point_index: Option<usize>,
                           state: State, x: i32, y:i32) -> Option<usize> {
    match state {
        State::Create => {
            add_node(vertices,points,x,y);
        }
        State::Edit => {
            let position = Vector2f::new(x as f32,y as f32);

            match selected_point_index {
                Some(index) => {
                    selected_point_index = None;
                    let radius = points[index].radius();

                    points[index].set_radius(radius / 2.0);

                },
                None => selected_point_index = get_selected_point_index(vertices,position)
            }

            select_point(selected_point_index,points);
        }
    }
    return selected_point_index;
}

pub fn mouse_move_handler(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                          bezier_curve: &mut BezierCurve,mut selected_point_index: Option<usize>,
                          state: State, x: i32, y:i32) {

    //check if we are in correct state.
    if state != State::Edit || selected_point_index.is_none() {return}
    let point_index = selected_point_index.unwrap();

    update_node_position(vertices,points,bezier_curve,point_index,x,y);


}

pub fn start_creating_new_curve(vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                                bezier_curve: &mut BezierCurve,state: State, mut animating: bool) -> State {
    if state == State::Create {return state}
    vertices.clear();
    points.clear();
    bezier_curve.clear();
    animating = false;

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