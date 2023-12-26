use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use super::sidebar;

const SIDEBAR_BUTTON_HEIGHT: Val = Val::Px(50.0);
const CHAPTER_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(4.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
const SECTION_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(16.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
const HIDDEN_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(0.0),
    right: Val::Px(0.0),
    top: Val::Px(0.0),
    bottom: Val::Px(0.0),
};

#[derive(Component, Copy, Clone)]
pub struct ChapterNumber(pub u32);

#[derive(Component)]
pub struct SectionNumber(pub u32);

#[derive(Component)]
pub struct ChapterContainer();

#[derive(Component)]
pub struct ChapterButton();

#[derive(Component)]
pub struct SectionContainer();

#[derive(Component)]
pub struct SidebarItem();

#[derive(Event, Debug)]
pub struct SidebarSwiperColorEvent(pub Color);

#[derive(Event)]
pub struct SectionVisibilityEvent(pub u32);

// true iff the chapter's sections are shown
#[derive(Component)]
pub struct ShowingSectionsOfThisChapter(bool);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectionVisibilityEvent>()
            .add_systems(Update, (
                section_button_visibility_system,
                sidebar_button_mouse_scroll,
                chapter_button_interaction
            ));
    }
}

pub fn chapter_button(commands: &mut Commands, chapter_name: &String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_button = (
        ChapterContainer(),
        SidebarItem(),
        chapter_number,
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: SIDEBAR_BUTTON_HEIGHT,
                border: CHAPTER_BUTTON_BORDER,
                padding: CHAPTER_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        }
    );

    let chapter_button = commands.spawn(chapter_button).id();
    // let chapter_button = chapter_button(commands, chapter_name, chapter_number);

    let text_item = (
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: 20.,
                
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let text_item = commands.spawn(text_item).id();
    commands.entity(chapter_button).push_children(&[text_item]);

    // commands.entity(chapter_button).push_children(&[chapter_button]);
    return chapter_button;
}

pub fn section_button(commands: &mut Commands, chapter_name: &String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_button = (
        SectionContainer(),
        SidebarItem(),
        chapter_number,
        ShowingSectionsOfThisChapter(true),
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: SIDEBAR_BUTTON_HEIGHT,
                border: SECTION_BUTTON_BORDER,
                padding: SECTION_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        }
    );

    let chapter_button = commands.spawn(chapter_button).id();

    let text_item = (
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: 20.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let text_item = commands.spawn(text_item).id();
    commands.entity(chapter_button).push_children(&[text_item]);

    return chapter_button;
}

fn section_button_visibility_system (
    mut section_button_query: Query<(&mut Visibility, &mut Style, &mut ShowingSectionsOfThisChapter, &ChapterNumber), With<SectionContainer>>,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionContainer>>,
    mut section_button_visibility_event: EventReader<SectionVisibilityEvent>,
) {
    for event in section_button_visibility_event.read() {
        for (mut section_button_visibility, mut style, mut showing_sections, section_button_chapter_number) in &mut section_button_query.iter_mut() {
            let chapter_button_chapter_number: u32 = event.0;
            let section_button_chapter_number: u32 = section_button_chapter_number.0;

            if chapter_button_chapter_number == section_button_chapter_number {
                println!("Pressed Chapter number {}", chapter_button_chapter_number);

                match showing_sections.0 {
                    true => { // Hide section if it's currently shown
                        *section_button_visibility = Visibility::Hidden;
                        style.height = Val::Px(0.0);
                        style.border = HIDDEN_BUTTON_BORDER;
                        style.padding = HIDDEN_BUTTON_BORDER;
                    }
                    false => { // show section if it's currently hidden
                    *section_button_visibility = Visibility::Inherited;
                    style.height = SIDEBAR_BUTTON_HEIGHT;
                    style.border = SECTION_BUTTON_BORDER;
                    style.padding = SECTION_BUTTON_BORDER;
                    }
                }
                showing_sections.0 = !showing_sections.0;
            }
        }
    }
}

fn sidebar_button_mouse_scroll(
    mut interaction_query: Query<
        &Interaction,
        With<SidebarItem>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
            }
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, sidebar_list_parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(sidebar_list_parent.get()).unwrap().size().y;
            
                        let max_scroll = (items_height - container_height).max(0.);
            
                        let dy = match mouse_wheel_event.unit {
                            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                            MouseScrollUnit::Pixel => mouse_wheel_event.y,
                        };
                        
                        scrolling_list.position += dy;
                        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling_list.position);
                    }
                }
            }
            Interaction::None => {
            }
        }
    }
}

fn chapter_button_interaction (
    mut interaction_query: Query<
        (&Interaction, &ChapterNumber, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SidebarItem>)
    >,
    mut section_visibility_writer: EventWriter<SectionVisibilityEvent>,
) {
    for (interaction, chapter_number, mut chapter_button_background_color, mut chapter_button_border_color ) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *chapter_button_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                section_visibility_writer.send(SectionVisibilityEvent(chapter_number.0));
            }
            Interaction::Hovered => {
                *chapter_button_background_color = Color::rgb(0.6, 0.6, 0.9).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
            Interaction::None => {
                *chapter_button_background_color = Color::rgb(0.5, 0.5, 0.5).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}