use bevy::{prelude::*, window::WindowResized};

use crate::ui::widgets::{window::WindowWidget, toolbar_item::ToolbarItemWidget};

pub mod window;
pub mod toolbar_item;

#[bevy_trait_query::queryable]
pub trait Widget {
    fn build(&mut self, commands: &mut Commands, scaling: Vec2);
    fn rebuild(&mut self);
}

pub struct CustomUIPlugin;

#[derive(Resource)]
struct UIScaling(Vec2);

impl Plugin for CustomUIPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app
            .register_component_as::<dyn Widget, WindowWidget>()
            .register_component_as::<dyn Widget, ToolbarItemWidget>()
            .add_systems(PreUpdate, update_widgets_system);
    }
}

fn update_widgets_system(
    mut commands: Commands, 
    mut widget_query: Query<&mut dyn Widget>,
    ev_resized: Res<Events<WindowResized>>,
) {

    let mut reader = ev_resized.get_reader();

    let mut scale = Vec2::ONE;

    for r in reader.read(&ev_resized) {
        // scale = Vec2 { x:  1. / 1920. * r.width, y: 1. / 1080. * r.height };
        scale = Vec2::ONE * 1. / 1080. * r.height; //scale to height
        commands.insert_resource(UIScaling(scale));
        for mut q in widget_query.iter_mut().flatten() {
            q.rebuild();
            println!("Draw!");
        }
    }

    for mut q in widget_query.iter_mut().flatten() {
        q.build(&mut commands, scale);
    }
}

#[derive(Component)]
pub struct WidgetNode(Option<Entity>);
