use std::time::Instant;

use bevy::prelude::*;
use bevy_egui::EguiContext;

use crate::{movement::Movement, Ship};

#[derive(Debug)]
pub struct SelectedShip {
    pub entity: Entity,
}

pub fn controls_ui(
    mut egui_context: ResMut<EguiContext>,
    mut selected_ship: ResMut<Option<SelectedShip>>,
    ships: Query<Entity, With<Ship>>,
) {
    // TODO: Handle "unselected" cases.
    if let Some(selected_ship) = &mut *selected_ship {
        egui::Window::new("Selected ship").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("Identifier: {:?}", *selected_ship));

            egui::ComboBox::from_id_source("selected_ship")
                .selected_text("-")
                .show_ui(ui, |ui| {
                    for entity in ships.iter() {
                        ui.selectable_value(
                            &mut selected_ship.entity,
                            entity,
                            format!("{:?}", entity),
                        );
                    }
                });
        });
    }
}

/// Controls current ship's movement with mouse keys.
pub fn mouse_clicks(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    ships: Query<(Entity, &Transform), With<Ship>>,
    selected_ship: Res<Option<SelectedShip>>,
) {
    let window = windows.get_primary().unwrap();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let position = (
                -(window.width() / 2. - position.x),
                -(window.height() / 2. - position.y),
            );
            info!("fly {:?}", position);

            let selected_ship = selected_ship
                .as_ref()
                .as_ref()
                .and_then(|s| ships.get(s.entity).ok());

            if let Some((ship, transform)) = selected_ship {
                commands.entity(ship).insert(Movement {
                    start_point: transform.translation,
                    when_started: Instant::now(),
                    destination: Vec3::new(position.1, -position.0, 0.),
                });
            }
        }
    }
}
