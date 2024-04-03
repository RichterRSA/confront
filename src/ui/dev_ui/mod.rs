use bevy::prelude::*;

use super::widgets::{window::{WindowWidgetBundle, WindowWidget}, toolbar_item::{ToolbarItemWidgetBundle, ToolbarItemWidget}};

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_ui);
    }
}

fn spawn_main_ui(
    mut commands: Commands,
){
    commands.spawn(WindowWidgetBundle{
        window_widget: WindowWidget {
            position: Vec2::ONE * 100.,
            size: Vec2 { x: 560., y: 80. },
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            ..Default::default()
        }
    }).with_children(|builder|{
        builder.spawn(ToolbarItemWidgetBundle{
            toolbar_item_widget: ToolbarItemWidget {
                size: Vec2::ONE * 80.,
                ..Default::default()
            }
        });
    });
}