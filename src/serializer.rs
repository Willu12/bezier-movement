use std::fs::File;
use std::io::{LineWriter, Write};
use sfml::graphics::{CircleShape, Vertex};
use crate::bezier_curve::BezierCurve;
use std::fs::read_to_string;
use sfml::system::Vector2f;
use crate::plain::{add_node, update_bezier};


pub fn save_polyline(vertices: &Vec<Vertex>) {
    //create file Data/polyline.txt
    let filename = "Data/polyline.txt";
    let mut file = LineWriter::new(File::create(filename).expect("File already exists"));

    for vertex in vertices.iter() {
        file.write((vertex.position.x.to_string() + " " + &*vertex.position.y.to_string() +"\n").as_bytes()).expect("cant write");
    }
}

pub fn load_polyline(file_path: &str, vertices: &mut Vec<Vertex>, points: &mut Vec<CircleShape>,
                     bezier_curve: &mut BezierCurve) {
    vertices.clear();
    points.clear();
    bezier_curve.clear();

    let positions = load_vertices_from_file(file_path);

    for position in positions {
        add_node(vertices,points,position.x as i32,position.y as i32);
    }

    update_bezier(bezier_curve,vertices);
}

fn load_vertices_from_file(file_path: &str) -> Vec<Vector2f> {
    let mut vertices: Vec<Vector2f> = vec![];

    let binding = read_to_string(file_path).expect("there is no file to load");
    let lines = binding.lines();

    for line in lines {
        if line.is_empty() {break}
        let mut iter = line.split_whitespace();

        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();


        vertices.push(Vector2f::new(x,y));
    }

    vertices
}