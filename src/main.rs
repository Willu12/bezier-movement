mod bezier_curve;
mod plain;
mod event_handlers;
mod image;

use egui_sfml::{egui, SfEgui};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style},
};
use sfml::graphics::{CircleShape, Vertex};
use sfml::system::Vector2f;
use crate::bezier_curve::BezierCurve;
use crate::event_handlers::{build_new_curve, mouse_click_handler, start_creating_new_curve};
use crate::image::{Animation, RotationKind};
use crate::image::Animation::{Movement, Rotation};
use crate::plain::{render_points, render_polyline, State};

fn main() {

    //init variables
    let mut vertices: Vec<Vertex> = vec![];
    let mut points: Vec<CircleShape> = vec![];
    let mut bezier_curve: BezierCurve= BezierCurve::new();
    let mut state: State = State::Edit;
    let mut selected_node_index: usize;
   // let mut image: image::Image = image::Image::new("Data/jeden.png",Vector2f::new(-1000.0,-1000.0));
    let mut animating: bool = false;
    let mut rotation_kind: RotationKind = RotationKind::Naive;

    let mut rw = RenderWindow::new(
        (800, 600),
        "krzeslo",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    // Step 1: Create an SfEgui
    let mut sfegui = SfEgui::new(&rw);


    while rw.is_open() {
        //check if we changed
        if state == State::Edit && animating {

            bezier_curve.do_frame();

        }


        while let Some(event) = rw.poll_event() {
            // Step 2: Collect events from the event loop
            sfegui.add_event(&event);
            match event {
                Event::Closed => {
                    rw.close();
                }
                Event::MouseButtonReleased {button:_, x,y} => {
                    mouse_click_handler(&mut vertices, &mut points,&mut bezier_curve,state,x,y);
                }
                _ => {}
            }
        }
        // Step 3: Do an egui frame with the desired ui function
        sfegui
            .do_frame(|ctx| {
                let win = egui::Window::new("Pierogi z jagodami");
                win.show(ctx, |ui| {

                    ui.vertical(|ui| {

                        ui.label("rotation kind");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut rotation_kind,RotationKind::Naive,"naive");
                            ui.radio_value(&mut rotation_kind,RotationKind::WithFiltering,"with filters");
                        });

                        ui.label("Animation kind");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut bezier_curve.image.animation,Movement,"movement");
                            ui.radio_value(&mut bezier_curve.image.animation,Rotation(rotation_kind),"Rotation");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("Create new curve").clicked() {
                                state = start_creating_new_curve(&mut vertices, &mut points, &mut bezier_curve, state, animating);
                            }
                            if ui.button("Finish creating curve").clicked() {
                                state = build_new_curve(&mut vertices, &mut points, &mut bezier_curve, state);
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("start Animation").clicked() {
                                animating = true;
                            }
                            if ui.button("stop Animation").clicked() {
                                animating = false;
                            }
                        });
                    });
                });
            })
            .unwrap();
        // Step 4: Draw
        rw.clear(Color::rgb(0, 0, 0));

        if state == State::Edit {
            bezier_curve.render(&mut rw);
        }


        render_points(&points,&mut rw);
        render_polyline(&vertices,&mut rw);



        sfegui.draw(&mut rw, None);
        rw.display();
    }
}