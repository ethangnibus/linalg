use std::thread::current;

use super::routes;
use super::sidebar;
use super::under_navbar;
use super::util::style;
use super::util::theme::background_color;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::FocusPolicy,
    // winit::WinitSettings,
};
use super::util::theme;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectionVisibilityEvent>()
            .add_event::<SubsectionVisibilityEvent>()
            .add_event::<ChapterButtonColorEvent>()
            .add_event::<ChapterButtonColorFunctionEvent>()
            .add_systems(
                Update,
                (
                    section_button_visibility_system,
                    subsection_button_visibility_system,
                    chapter_button_interaction,
                    section_button_interaction,
                    subsection_button_interaction,
                    sidebar_button_mouse_scroll,
                    header_button_color_change_system,
                    chapter_button_text_color_system,
                    chapter_button_line_color_system,
                    chapter_button_expander_text_system,
                    chapter_button_line_color_function_system,
                    chapter_button_text_color_function_system,
                ),
            );
    }
}

pub const HEADER_BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const TITLE_BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const SIDEBAR_BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const HIDDEN_SIDEBAR_BUTTON_HEIGHT: Val = Val::Px(0.0);
pub const CHAPTER_BUTTON_FONT_SIZE: f32 = 18.0;
pub const SECTION_BUTTON_FONT_SIZE: f32 = 16.0;
pub const SUBSECTION_BUTTON_FONT_SIZE: f32 = 14.0;

// i-th index gets the number of sections in the i-th chapter. For example
// NUMBER_OF_SECTIONS_IN_CHAPTER[3] gets the number of sections in chapter 3
const NUMBER_OF_SECTIONS_IN_CHAPTER: [u32; 16] = [
    0, 4, // Chapter 1
    4, // Chapter 2
    4, // Chapter 3
    4, // Chapter 4
    3, // Chapter 5
    3, // Chapter 6
    4, // Chapter 7
    3, // Chapter 8
    2, // Chapter 9
    3, // Chapter 10
    3, // Chapter 11
    3, // Chapter 12
    3, // Chapter 13
    3, // Chapter 14
    0, // Bibliography
];

pub const HEADER_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(0.0),
    right: Val::Px(0.0),
    top: Val::Px(0.0),
    bottom: Val::Px(0.0),
};
pub const TITLE_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(4.0),
    right: Val::Px(4.0),
    top: Val::Px(20.0),
    bottom: Val::Px(4.0),
};
pub const CHAPTER_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(12.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
pub const SECTION_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(20.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
pub const SUBSECTION_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(28.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
pub const HIDDEN_BUTTON_BORDER: UiRect = style::NO_BORDER;

#[derive(Component, Copy, Clone)]
pub struct ChapterNumber(pub u32);

#[derive(Component, Copy, Clone)]
pub struct SectionNumber(pub u32);

#[derive(Component, Copy, Clone)]
pub struct SubsectionNumber(pub u32);

#[derive(Component)]
pub struct HeaderButton();

#[derive(Component)]
pub struct ChapterButton();

#[derive(Component)]
pub struct SectionButton();

#[derive(Component)]
pub struct SubsectionButton();

#[derive(Component)]
pub struct ChapterButtonText;

#[derive(Component)]
pub struct ChapterButtonLine;

#[derive(Component)]
pub struct ChapterButtonExpanderText;

#[derive(Component)]
pub struct SectionButtonText;

#[derive(Component)]
pub struct SubsectionButtonText;

#[derive(Component)]
pub struct SidebarItem();

#[derive(Event)]
pub struct SectionVisibilityEvent {
    pub chapter_number: u32,
}

#[derive(Event)]
pub struct SubsectionVisibilityEvent {
    pub chapter_number: u32,
    pub section_number: u32,
    pub showing_sections: bool,
}

// true iff the chapter's sections are shown
#[derive(Component)]
pub struct ShowingSectionsOfThisChapter(bool);

// true iff the section's subsections are shown
#[derive(Component)]
pub struct ShowingSubsectionsOfThisSection(bool);


// ================================
// ========== UI Buttons ==========
// ================================
pub fn header_button(commands: &mut Commands, theme: &theme::CurrentTheme, text: &String) -> Entity {
    let header_button = (
        SidebarItem(),
        HeaderButton(),
        theme::ColorFunction {
            background: theme::sidebar_color,
            border: theme::sidebar_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                min_height: HEADER_BUTTON_HEIGHT,
                // height: Val::Percent(100.0),
                border: HEADER_BUTTON_BORDER,
                padding: HEADER_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::sidebar_color(theme).into(),
            ..default()
        },
    );

    let title_button = commands.spawn(header_button).id();
    // let chapter_button = chapter_button(commands, chapter_name, chapter_number);

    let text_item = (
        theme::ColorFunction {
            background: theme::sidebar_header_text_color,
            border: theme::sidebar_header_text_color,
        },
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: CHAPTER_BUTTON_FONT_SIZE,
                color: theme::sidebar_header_text_color(theme),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );

    // let bottom_line = (
    //     SidebarItem(),
    //     ButtonBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Px(1.0),
    //             border: HIDDEN_BUTTON_BORDER,
    //             padding: HIDDEN_BUTTON_BORDER,
    //             justify_content: JustifyContent::Center,
    //             align_content: AlignContent::Center,
    //             flex_direction: FlexDirection::Column,
    //             ..default()
    //         },
    //         background_color: Color::rgb(1.0, 1.0, 1.0).into(),
    //         // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
    //         ..default()
    //     }
    // );
    // let bottom_line = commands.spawn(bottom_line).id();

    let text_item = commands.spawn(text_item).id();
    commands.entity(title_button).push_children(&[text_item]);

    // commands.entity(chapter_button).push_children(&[chapter_button]);
    return title_button;
}

pub fn title_button(commands: &mut Commands, theme: &theme::CurrentTheme, text: &String) -> Entity {
    let title_button = (
        SidebarItem(),
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                min_height: TITLE_BUTTON_HEIGHT,
                border: TITLE_BUTTON_BORDER,
                padding: TITLE_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::background_color(theme).into(),
            ..default()
        },
    );

    let title_button = commands.spawn(title_button).id();
    // let chapter_button = chapter_button(commands, chapter_name, chapter_number);

    let text_item = (
        theme::ColorFunction {
            background: theme::sidebar_color,
            border: theme::sidebar_color,
        },
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: CHAPTER_BUTTON_FONT_SIZE,
                color: theme::sidebar_color(theme).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let text_item = commands.spawn(text_item).id();

    let bottom_line = (
        SidebarItem(),
        theme::ColorFunction {
            background: theme::sidebar_color,
            border: theme::sidebar_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::sidebar_color(theme).into(),
            // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
    );
    let bottom_line = commands.spawn(bottom_line).id();

    // commands.entity(title_button).push_children(&[text_item, bottom_line]);
    commands
        .entity(title_button)
        .push_children(&[text_item, bottom_line]);
    // commands.entity(chapter_button).push_children(&[chapter_button]);
    return title_button;
}

// ---------- Chapter UI Button ----------
pub fn chapter_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme, 
    chapter_name: &String,
    chapter_number: u32,
) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_button = commands.spawn((
        ChapterButton(),
        SidebarItem(),
        chapter_number,
        ShowingSectionsOfThisChapter(false),
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                min_height: SIDEBAR_BUTTON_HEIGHT,
                border: CHAPTER_BUTTON_BORDER,
                padding: CHAPTER_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        },
    )).id();

    let text_holder = commands.spawn((NodeBundle {
        style: Style {
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    })).id();

    let text_item = commands.spawn((
        ChapterButtonText,
        chapter_number,
        theme::ColorFunction {
            background: theme::sidebar_collapsed_color,
            border: theme::sidebar_collapsed_color,
        },
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: CHAPTER_BUTTON_FONT_SIZE,
                color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    let expander_text = commands.spawn((
        ChapterButtonText,
        ChapterButtonExpanderText,
        chapter_number,
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        TextBundle::from_section(
            "+ ",
            TextStyle {
                font_size: CHAPTER_BUTTON_FONT_SIZE,
                color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
        ),
    )).id();

    let bottom_line = commands.spawn((
        SidebarItem(),
        ChapterButtonLine,
        chapter_number,
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::text_color(theme).into(),
            // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
    )).id();

    // commands.entity(chapter_button).push_children(&[part_flag, text_item]);
    commands.entity(text_holder).push_children(&[expander_text, text_item]);

    commands
        .entity(chapter_button)
        .push_children(&[text_holder, bottom_line]);

    return chapter_button;
}

// ---------- Section UI Button ----------
pub fn section_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme, 
    chapter_name: &String,
    chapter_number: u32,
    section_number: u32,
) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let section_number = SectionNumber(section_number);
    let section_button = (
        SectionButton(),
        SidebarItem(),
        section_number.clone(),
        chapter_number,
        ShowingSectionsOfThisChapter(false),
        ShowingSubsectionsOfThisSection(false),
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: HIDDEN_SIDEBAR_BUTTON_HEIGHT,
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            visibility: Visibility::Hidden,
            focus_policy: FocusPolicy::Block,
            ..default()
        },
    );

    let section_button = commands.spawn(section_button).id();

    let text_item = (
        SectionButtonText,
        section_number,
        chapter_number,
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: SECTION_BUTTON_FONT_SIZE,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let text_item = commands.spawn(text_item).id();

    let bottom_line = (
        SidebarItem(),
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::text_color(theme).into(),
            ..default()
        },
    );
    let bottom_line = commands.spawn(bottom_line).id();

    commands
        .entity(section_button)
        .push_children(&[text_item, bottom_line]);

    return section_button;
}

// ---------- Subsection UI Button ----------
pub fn subsection_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme, 
    chapter_name: &String,
    chapter_number: u32,
    section_number: u32,
    subsection_number: u32,
) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let section_number = SectionNumber(section_number);
    let subsection_number = SubsectionNumber(subsection_number);

    let subsection_button = (
        SubsectionButton(),
        SidebarItem(),
        chapter_number,
        section_number,
        subsection_number,
        ShowingSectionsOfThisChapter(false),
        ShowingSubsectionsOfThisSection(false),
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: HIDDEN_SIDEBAR_BUTTON_HEIGHT,
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            focus_policy: FocusPolicy::Block,
            visibility: Visibility::Hidden,
            ..default()
        },
    );

    let subsection_button = commands.spawn(subsection_button).id();

    let text_item = (
        SubsectionButtonText,
        chapter_number,
        section_number,
        subsection_number,
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: SUBSECTION_BUTTON_FONT_SIZE,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let text_item = commands.spawn(text_item).id();

    let bottom_line = (
        SidebarItem(),
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: theme::text_color(theme).into(),
            // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
    );
    let bottom_line = commands.spawn(bottom_line).id();

    commands
        .entity(subsection_button)
        .push_children(&[text_item, bottom_line]);

    return subsection_button;
}

// =========================================
// ========== Interaction Systems ==========
// =========================================


// ---------- Header Button ----------
fn header_button_color_change_system(
    mut header_button_query: Query<&mut BackgroundColor, With<HeaderButton>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<under_navbar::SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_swiper_color_event_reader.read() {
        for mut header_button_color in &mut header_button_query.iter_mut() {
            let color = event.0;
            if color != theme::NOT_A_COLOR {
                *header_button_color = color.into();
            }
        }

    }
}

// ---------- Chapter Interaction ----------
#[derive(Event)]
pub struct ChapterButtonColorEvent {
    color: Color,
    chapter_number: u32,
}

fn chapter_button_text_color_system(
    mut chapter_button_text_color_reader: EventReader<ChapterButtonColorEvent>,
    mut text_query: Query<(&mut Text, &ChapterNumber), With<ChapterButtonText>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in chapter_button_text_color_reader.read() {
        
        for (mut text, chapter_number) in text_query.iter_mut() {
            // text.sections[0].style.color = theme::sidebar_color(&theme);
            if chapter_number.0 == event.chapter_number {
                text.sections[0].style.color = event.color;
            }
        }
    }
}

fn chapter_button_text_color_function_system(
    mut chapter_button_color_function_reader: EventReader<ChapterButtonColorFunctionEvent>,
    mut text_query: Query<(&mut theme::ColorFunction, &ChapterNumber), With<ChapterButtonText>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in chapter_button_color_function_reader.read() {
        for (mut color_function, chapter_number) in text_query.iter_mut() {
            if chapter_number.0 == event.chapter_number {
                color_function.background = event.color_function;
                color_function.border = event.color_function;
            }
        }
    }
}

fn chapter_button_line_color_system(
    mut chapter_button_text_color_reader: EventReader<ChapterButtonColorEvent>,
    mut text_query: Query<(&mut BackgroundColor, &ChapterNumber), With<ChapterButtonLine>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in chapter_button_text_color_reader.read() {
        
        for (mut background_color, chapter_number) in text_query.iter_mut() {
            if chapter_number.0 == event.chapter_number {
                *background_color = event.color.into();
            }
        }
    }
}

fn chapter_button_line_color_function_system(
    mut chapter_button_color_function_reader: EventReader<ChapterButtonColorFunctionEvent>,
    mut line_query: Query<(&mut theme::ColorFunction, &ChapterNumber), With<ChapterButtonLine>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in chapter_button_color_function_reader.read() {
        for (mut color_function, chapter_number) in line_query.iter_mut() {
            if chapter_number.0 == event.chapter_number {
                color_function.background = event.color_function;
                color_function.border = event.color_function;
            }
        }
    }
}

fn chapter_button_expander_text_system(
    mut chapter_button_expander_text_reader: EventReader<SectionVisibilityEvent>,
    mut expander_text_query: Query<(&mut Text, &ChapterNumber), With<ChapterButtonExpanderText>>,
) {
    for event in chapter_button_expander_text_reader.read() {
        for (mut text, chapter_number) in expander_text_query.iter_mut() {
            if chapter_number.0 != event.chapter_number { continue };
            if text.sections[0].value == "+ " {
                text.sections[0].value = String::from("- ");
            } else if text.sections[0].value == "- " {
                text.sections[0].value = String::from("+ ");
            }
        
        }
    }
}


#[derive(Event)]
pub struct ChapterButtonColorFunctionEvent {
    chapter_number: u32,
    color_function: fn(&theme::CurrentTheme) -> Color,
}

fn chapter_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &ChapterNumber,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut ShowingSectionsOfThisChapter,
        ),
        (Changed<Interaction>, With<ChapterButton>),
    >,
    mut expander_text_query: Query<(&Text, &ChapterNumber, &mut theme::ColorFunction), With<ChapterButtonExpanderText>>,
    // mut text_querxy: Query<(&ChapterNumber, &mut theme::ColorFunction), With<ChapterButtonText>>,
    mut chapter_button_text_color_writer: EventWriter<ChapterButtonColorEvent>,
    mut section_visibility_writer: EventWriter<SectionVisibilityEvent>,
    mut chapter_button_color_function_writer: EventWriter<ChapterButtonColorFunctionEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    for (
        interaction,
        chapter_number,
        mut chapter_button_background_color,
        mut chapter_button_border_color,
        mut showing_sections,
    ) in &mut interaction_query
    {
        let mut idle_color: Color = Color::default();
        let mut hovered_color: Color = Color::default();

        // Set expander text color for later use
        for (
            text,
            query_chapter_number,
            mut expander_color_function
        ) in expander_text_query.iter_mut() {
            if query_chapter_number.0 != chapter_number.0 { continue };

            if text.sections[0].value == "+ " {
                hovered_color = theme::sidebar_color(&theme);
                expander_color_function.background = theme::sidebar_collapsed_color;
                expander_color_function.border = theme::sidebar_collapsed_color;
                idle_color = theme::sidebar_collapsed_color(&theme);
                chapter_button_color_function_writer.send(
                    ChapterButtonColorFunctionEvent {
                        chapter_number: chapter_number.0,
                        color_function: theme::sidebar_collapsed_color,
                    }
                )
            } else if text.sections[0].value == "- " {
                hovered_color = theme::sidebar_collapsed_color(&theme);
                expander_color_function.background = theme::sidebar_color;
                expander_color_function.border = theme::sidebar_color;
                idle_color = theme::sidebar_color(&theme);
                chapter_button_color_function_writer.send(
                    ChapterButtonColorFunctionEvent {
                        chapter_number: chapter_number.0,
                        color_function: theme::sidebar_color,
                    }
                )
            }
        }

        // Set expander text color for later use
        // for (
        //     text,
        //     query_chapter_number,
        //     mut color_function
        // ) in expander_text_query.iter_mut() {
        //     if query_chapter_number.0 != chapter_number.0 { continue };

        //     if text.sections[0].value == "+ " {
        //         hovered_color = theme::sidebar_color(&theme);
        //         color_function.background = theme::sidebar_collapsed_color;
        //         color_function.border = theme::sidebar_collapsed_color;
        //         idle_color = theme::sidebar_collapsed_color(&theme);
        //     } else if text.sections[0].value == "- " {
        //         hovered_color = theme::sidebar_collapsed_color(&theme);
        //         color_function.background = theme::sidebar_color;
        //         color_function.border = theme::sidebar_color;
        //         idle_color = theme::sidebar_color(&theme);
        //     }
        // }

        match *interaction {
            Interaction::Pressed => {
                // *chapter_button_background_color = pressed_color;
                // *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                section_visibility_writer.send(SectionVisibilityEvent {
                    chapter_number: chapter_number.0,
                });
                showing_sections.0 = !showing_sections.0;
            }
            Interaction::Hovered => {
                // *chapter_button_background_color = hovered_color;
                // *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                // chapter_button_text_color_writer.send(ChapterButtonColorEvent);
                chapter_button_text_color_writer.send(ChapterButtonColorEvent{
                    // color: theme::sidebar_collapsed_color(&theme),
                    color: hovered_color,
                    chapter_number: chapter_number.0,
                });
            }
            Interaction::None => {
                // *chapter_button_background_color = Color::rgb(0.1, 0.1, 0.1).into();
                // *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                chapter_button_text_color_writer.send(ChapterButtonColorEvent{
                    color: idle_color,
                    chapter_number: chapter_number.0,
                });
            }
        }
    }
}

// ---------- Section Interaction ----------
fn section_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &ChapterNumber,
            &SectionNumber,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut ShowingSubsectionsOfThisSection,
        ),
        (Changed<Interaction>, With<SectionButton>),
    >,
    mut subsection_visibility_writer: EventWriter<SubsectionVisibilityEvent>,
) {
    for (
        interaction,
        chapter_number,
        section_number,
        mut chapter_button_background_color,
        mut chapter_button_border_color,
        mut showing_subsections,
    ) in &mut interaction_query
    {
        let chapter_number: u32 = chapter_number.0;
        let section_number: u32 = section_number.0;

        let mut pressed_color: BackgroundColor = Color::default().into();
        let mut hovered_color: BackgroundColor = Color::default().into();

        match showing_subsections.0 {
            false => {
                pressed_color = Color::rgb(0.45, 0.45, 0.7).into();
                hovered_color = Color::rgb(0.6, 0.6, 0.9).into();
            }
            true => {
                pressed_color = Color::rgb(0.7, 0.45, 0.45).into();
                hovered_color = Color::rgb(0.9, 0.6, 0.6).into();
            }
        }

        match *interaction {
            Interaction::Pressed => {
                *chapter_button_background_color = pressed_color;
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                subsection_visibility_writer.send(SubsectionVisibilityEvent {
                    chapter_number: chapter_number,
                    section_number: section_number,
                    showing_sections: true,
                });
                showing_subsections.0 = !showing_subsections.0;
            }
            Interaction::Hovered => {
                *chapter_button_background_color = hovered_color;
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
            Interaction::None => {
                *chapter_button_background_color = Color::rgb(0.1, 0.1, 0.1).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}

// ---------- Subsection Interaction ----------
fn subsection_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &ChapterNumber,
            &SectionNumber,
            &SubsectionNumber,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<SubsectionButton>),
    >,
    mut routing_event_writer: EventWriter<routes::RoutingEvent>,
    current_route: Res<routes::CurrentRoute>,
    // mut section_visibility_writer: EventWriter<SectionVisibilityEvent>,
) {
    for (
        interaction,
        chapter_number,
        section_number,
        subsection_number,
        mut chapter_button_background_color,
        mut chapter_button_border_color,
    ) in &mut interaction_query
    {
        let chapter_number: u32 = chapter_number.0;
        let section_number: u32 = section_number.0;
        let subsection_number: u32 = subsection_number.0;

        match *interaction {
            Interaction::Pressed => {
                *chapter_button_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                // section_visibility_writer.send(SectionVisibilityEvent(chapter_number.0));

                if current_route.chapter_number != chapter_number
                    || current_route.section_number != section_number
                    || current_route.subsection_number != subsection_number
                {
                    routing_event_writer.send(routes::RoutingEvent {
                        chapter_number: chapter_number,
                        section_number: section_number,
                        subsection_number: subsection_number,
                    });
                }
            }
            Interaction::Hovered => {
                *chapter_button_background_color = Color::rgb(0.6, 0.6, 0.9).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
            Interaction::None => {
                *chapter_button_background_color = Color::rgb(0.1, 0.1, 0.1).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}

// =========================================
// ========== Visibility Systems ===========
// =========================================

// ---------- Section Button Visibility ----------
fn section_button_visibility_system(
    mut section_button_query: Query<
        (
            &mut Visibility,
            &mut Style,
            &mut ShowingSectionsOfThisChapter,
            &ChapterNumber,
            &mut ShowingSubsectionsOfThisSection,
        ),
        With<SectionButton>,
    >,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionButton>>,
    mut section_button_visibility_event: EventReader<SectionVisibilityEvent>,
    mut subsection_visibility_writer: EventWriter<SubsectionVisibilityEvent>,
) {
    for event in section_button_visibility_event.read() {
        for (
            mut section_button_visibility,
            mut style,
            mut showing_sections,
            section_button_chapter_number,
            mut showing_subsections,
        ) in &mut section_button_query.iter_mut()
        {
            let chapter_button_chapter_number: u32 = event.chapter_number;
            let section_button_chapter_number: u32 = section_button_chapter_number.0;

            if chapter_button_chapter_number == section_button_chapter_number {
                match showing_sections.0 {
                    true => {
                        // Hide section if it's currently shown
                        *section_button_visibility = Visibility::Hidden;
                        style.height = HIDDEN_SIDEBAR_BUTTON_HEIGHT;
                        style.border = HIDDEN_BUTTON_BORDER;
                        style.padding = HIDDEN_BUTTON_BORDER;

                        let num_sections_in_chapter: u32 =
                            NUMBER_OF_SECTIONS_IN_CHAPTER[chapter_button_chapter_number as usize];
                        for section_number in 1..num_sections_in_chapter + 1 {
                            subsection_visibility_writer.send(SubsectionVisibilityEvent {
                                chapter_number: chapter_button_chapter_number,
                                section_number: section_number,
                                showing_sections: false,
                            });
                        }
                        showing_subsections.0 = false;
                    }
                    false => {
                        // show section if it's currently hidden
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

// ---------- Subsection Button Visibility ----------
fn subsection_button_visibility_system(
    mut subsection_button_query: Query<
        (
            &mut Visibility,
            &mut Style,
            &mut ShowingSubsectionsOfThisSection,
            &ChapterNumber,
            &SectionNumber,
        ),
        With<SubsectionButton>,
    >,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionButton>>,
    mut section_button_visibility_event: EventReader<SectionVisibilityEvent>,
    mut subsection_button_visibility_event: EventReader<SubsectionVisibilityEvent>,
) {
    for event in subsection_button_visibility_event.read() {
        for (
            mut subsection_button_visibility,
            mut style,
            mut showing_subsections,
            subsection_button_chapter_number,
            subsection_button_section_number,
        ) in &mut subsection_button_query.iter_mut()
        {
            // event where chapter hides subsections
            let event_section_number: u32 = event.section_number;
            let event_chapter_number: u32 = event.chapter_number;
            let event_showing_sections = event.showing_sections;

            let subsection_button_section_number: u32 = subsection_button_section_number.0;
            let subsection_button_chapter_number: u32 = subsection_button_chapter_number.0;

            if event_section_number == subsection_button_section_number
                && event_chapter_number == subsection_button_chapter_number
            {
                match showing_subsections.0 {
                    true => {
                        // Hide subsection if it's currently shown
                        match event_showing_sections {
                            true => {
                                // section is currently shown
                                *subsection_button_visibility = Visibility::Hidden;
                                style.height = HIDDEN_SIDEBAR_BUTTON_HEIGHT;
                                style.border = HIDDEN_BUTTON_BORDER;
                                style.padding = HIDDEN_BUTTON_BORDER;
                                showing_subsections.0 = false // make it so subsection is hidden
                            }
                            false => {
                                // section is currently hidden
                                showing_subsections.0 = false // make it so subsection is visible
                            }
                        }
                    }
                    false => {
                        // show subsection if it's currently hidden
                        match event_showing_sections {
                            // section is currently shown
                            true => {
                                *subsection_button_visibility = Visibility::Inherited;
                                style.height = SIDEBAR_BUTTON_HEIGHT;
                                style.border = SUBSECTION_BUTTON_BORDER;
                                style.padding = SUBSECTION_BUTTON_BORDER;
                                showing_subsections.0 = true // make it so subsection is visible
                            }
                            false => {
                                // section is currently hidden
                                *subsection_button_visibility = Visibility::Hidden;
                                style.height = HIDDEN_SIDEBAR_BUTTON_HEIGHT;
                                style.border = HIDDEN_BUTTON_BORDER;
                                style.padding = HIDDEN_BUTTON_BORDER;
                            }
                        }
                    }
                }
            }
        }
    }
}

// =========================================
// ========== Scrolling Systems ===========
// =========================================
fn sidebar_button_mouse_scroll(
    mut interaction_query: Query<&Interaction, With<SidebarItem>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, sidebar_list_parent, list_node) in
                        &mut query_list
                    {
                        let items_height = list_node.size().y;
                        let container_height =
                            query_node.get(sidebar_list_parent.get()).unwrap().size().y;

                        let max_scroll = (items_height - container_height).max(0.);

                        let dy = match mouse_wheel_event.unit {
                            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                            MouseScrollUnit::Pixel => mouse_wheel_event.y,
                        };

                        scrolling_list.position += dy * 0.8; // change the constant to change scroll speed
                        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling_list.position);
                    }
                }
            }
            Interaction::None => {}
        }
    }
}
