mod bezier_curve;
mod plain;
mod event_handlers;

use egui_sfml::{egui, SfEgui};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style},
};
use sfml::graphics::{CircleShape, Vertex};
use crate::bezier_curve::BezierCurve;
use crate::event_handlers::mouse_click_handler;
use crate::plain::{render_points, render_polyline, State};

fn main() {

    //init variables
    let mut vertices: Vec<Vertex> = vec![];
    let mut points: Vec<CircleShape> = vec![];
    let mut bezier_curve: BezierCurve= BezierCurve::new();
    let mut state: State = State::Create;
    let mut selected_node_index: usize;

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


        while let Some(event) = rw.poll_event() {
            // Step 2: Collect events from the event loop
            sfegui.add_event(&event);
            match event {
                Event::Closed => {
                    rw.close();
                }
                Event::MouseButtonReleased {button:_, x,y} => {
                    mouse_click_handler(&mut vertices, &mut points,&mut bezier_curve,&state,x,y);
                }
                _ => {}
            }
        }
        // Step 3: Do an egui frame with the desired ui function
        sfegui
            .do_frame(|ctx| {
                let win = egui::Window::new("Pierogi z jagodami");
                win.show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.vertical(|ui| {
                            ui.radio_value(&mut state,State::Create,"Create");
                            ui.radio_value(&mut state,State::Edit,"Edit");
                        });
                    });
                });
            })
            .unwrap();
        // Step 4: Draw
        rw.clear(Color::rgb(0, 0, 0));
        render_points(&points,&mut rw);
        render_polyline(&vertices,&mut rw);
        if state == State::Edit {
            bezier_curve.render(&mut rw);
        }


        sfegui.draw(&mut rw, None);
        rw.display();
    }
}