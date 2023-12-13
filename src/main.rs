mod bezier_curve;
mod plain;
mod event_handlers;
mod image;
mod transormations;
mod serializer;

use egui_sfml::{egui, SfEgui};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style},
};
use sfml::graphics::{CircleShape, Vertex};
use crate::bezier_curve::BezierCurve;
use crate::event_handlers::{build_new_curve, mouse_click_handler, mouse_move_handler, start_creating_new_curve};
use crate::image::{RotationKind};
use crate::image::Animation::{Movement, Rotation};
use crate::plain::{render_points, render_polyline, State};

fn main() {

    //init variables
    let mut vertices: Vec<Vertex> = vec![];
    let mut points: Vec<CircleShape> = vec![];
    let mut bezier_curve: BezierCurve= BezierCurve::new();
    let mut state: State = State::Edit;
    let mut selected_node_index: Option<usize> = None;
    let mut animating: bool = false;
    let mut rotation_kind: RotationKind = RotationKind::Naive;
    let mut polyline_visible: bool = true;

    let mut is_egui_clicked = false;

    let mut rw = RenderWindow::new(
        (800, 600),
        "move along bezier",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    // Step 1: Create an SfEgui
    let mut sfegui = SfEgui::new(&rw);

    while rw.is_open() {
        //check if we changed
        if state == State::Edit && animating && vertices.is_empty() == false {
            bezier_curve.do_frame();
        }

        while let Some(event) = rw.poll_event() {
            sfegui.add_event(&event);
            match event {
                Event::Closed => {
                    rw.close();
                }
                Event::MouseButtonReleased {button:_, x,y} => {
                    if is_egui_clicked == false {
                    selected_node_index =  mouse_click_handler(&mut vertices,
                                                              &mut points,
                                                              selected_node_index,
                                                              state,x,y,polyline_visible);
                    }
                }
                Event::MouseMoved {x,y} => {
                    mouse_move_handler(&mut vertices, &mut points, &mut bezier_curve,selected_node_index,state,x,y);
                }
                _ => {}
            }
        }
        // Step 3: Do an egui frame with the desired ui function
        sfegui
            .do_frame(|ctx| {
                let win = egui::Window::new("settings");
                is_egui_clicked = ctx.is_pointer_over_area();
                win.show(ctx, |ui| {

                    ui.vertical(|ui| {

                        ui.checkbox(&mut polyline_visible,"polyline visible");
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
                                state = start_creating_new_curve(&mut vertices, &mut points, &mut bezier_curve);
                                animating = false;
                            }
                            if ui.button("Finish creating curve").clicked() {
                                state = build_new_curve(&mut vertices, &mut bezier_curve, state);
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("save polyline").clicked() {
                                serializer::save_polyline(&vertices);
                            }
                            if ui.button("load polyline").clicked() {
                                serializer::load_polyline("Data/polyline.txt",&mut vertices,&mut points,& mut bezier_curve);
                                state = State::Edit;
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

        if polyline_visible || state == State::Create {
            render_points(&points, &mut rw);
            render_polyline(&vertices, &mut rw);
        }

        sfegui.draw(&mut rw, None);
        rw.display();
    }
}