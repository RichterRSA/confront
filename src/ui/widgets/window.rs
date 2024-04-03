use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Widget, WidgetNode};

#[derive(Component)]
pub struct WindowWidget {
    pub position: Vec2,
    pub size: Vec2,
    pub background_color: Color,
    pub node: WidgetNode,
    pub align_self: AlignSelf,
    pub justify_self: JustifySelf,
    pub built : bool,
}

impl Default for WindowWidget {
    fn default() -> Self {
        Self { position: Vec2::ZERO, size: Vec2::ONE * 100., background_color: Color::WHITE, node: WidgetNode(None), built: false, align_self: AlignSelf::Auto, justify_self: JustifySelf::Auto}
    }
}

#[derive(Bundle)]
pub struct WindowWidgetBundle {
    pub window_widget: WindowWidget,
}

impl Default for WindowWidgetBundle {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for WindowWidget {
    fn build(&mut self, commands: &mut Commands, scaling: Vec2) {
        if self.built {
            return;
        }

        match self.node.0 {
            Some(n) => {
                commands.entity(n).despawn_recursive();
            },
            None => {},
        }

        let back_col = self.background_color;

        let node = commands.spawn((
            NodeBundle{
                background_color: BackgroundColor(self.background_color),
                style: Style {
                    width: Val::Px(self.size.x * scaling.x),
                    height: Val::Px(self.size.y * scaling.y),
                    align_self: self.align_self,
                    justify_self: self.justify_self,
                    ..Default::default()
                },
                ..Default::default()
            },
            On::<Pointer<picking_core::events::Out>>::target_component_mut::<BackgroundColor>(move |_, col|{
                col.0 = back_col;
            }),
            On::<Pointer<picking_core::events::Over>>::target_component_mut::<BackgroundColor>(|_, col|{
                col.0 = Color::PINK;
            }),
            On::<Pointer<picking_core::events::Down>>::target_component_mut::<BackgroundColor>(|_, col|{
                col.0 = Color::BLUE;
            }),
            On::<Pointer<picking_core::events::Up>>::target_component_mut::<BackgroundColor>(|_, col|{
                col.0 = Color::PINK;
            }),
        )).id();
        self.node = WidgetNode(Some(node));
        self.built = true;
    }

    fn rebuild(&mut self) {
        self.built = false;
    }
}

impl WindowWidgetBundle {
    pub fn new() -> Self {
        Self {  window_widget: WindowWidget::default() }
    }
}