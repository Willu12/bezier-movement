mod point;
mod bezier_curve;
mod plain;

use egui_sfml::{egui, SfEgui};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style},
};

fn main() {



    let mut rw = RenderWindow::new(
        (800, 600),
        "Hello egui!",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    // Step 1: Create an SfEgui
    let mut sfegui = SfEgui::new(&rw);

    let mut name = String::new();
    let mut msg = String::new();

    while rw.is_open() {
        while let Some(event) = rw.poll_event() {
            // Step 2: Collect events from the event loop
            sfegui.add_event(&event);
            if matches!(event, Event::Closed) {
                rw.close();
            }
        }
        // Step 3: Do an egui frame with the desired ui function
        sfegui
            .do_frame(|ctx| {
                let win = egui::Window::new("Pierogi z jagodami");
                win.show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut name);
                        if ui.button("Say hello").clicked() {
                            msg = format!("Hello {name}!");
                        }
                    });
                    if !msg.is_empty() {
                        ui.separator();
                        ui.label(&msg);
                    }
                });
            })
            .unwrap();
        // Step 4: Draw
        rw.clear(Color::rgb(95, 106, 62));
        sfegui.draw(&mut rw, None);
        rw.display();
    }
}