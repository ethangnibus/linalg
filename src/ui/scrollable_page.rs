use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

// Marker for UI node
#[derive(Component)]
pub struct ScrollablePage;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}


// pub fn get_page() -> Entity {
//     // let scrollable_page = new();
//     // let scrollable_page = testing(&commands);
//     // return commands.spawn(scrollable_page).id();
//     let scrollable_page = new();

//     // let scrollable_page = commands.spawn(scrollable_page).id();

//     // commands.entity(scrollable_page).push_children(&page_items);
//     return scrollable_page;
// }

pub fn get_page() -> (ScrollablePage, NodeBundle, AccessibilityNode) {
    return (
        ScrollablePage,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center, // maybe remove
                // overflow: Overflow::clip(),
                // row_gap: Val::Percent(0.01),
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
        AccessibilityNode(NodeBuilder::new(Role::List)),
    );
}

fn temp() {}

// const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// const PRESSED_BUTTON: Color = Color::rgb(0.0, 0.0, 0.0);

// fn button_system(
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &Children,
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut color, mut border_color, children) in &mut interaction_query {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 text.sections[0].value = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = Color::RED;
//             }
//             Interaction::Hovered => {
//                 text.sections[0].value = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 text.sections[0].value = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = Color::BLACK;
//             }
//         }
//     }
// }

