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

const SIDEBAR_BUTTON_HEIGHT: Val = Val::Px(90.0);
const HIDDEN_SIDEBAR_BUTTON_HEIGHT: Val = Val::Px(0.0);

const CHAPTER_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(4.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
const SECTION_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(12.0),
    right: Val::Px(4.0),
    top: Val::Px(0.0),
    bottom: Val::Px(4.0),
};
const SUBSECTION_BUTTON_BORDER: UiRect = UiRect {
    left: Val::Px(20.0),
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
pub struct SubsectionNumber(pub u32);

#[derive(Component)]
pub struct ChapterButton();

#[derive(Component)]
pub struct SectionButton();

#[derive(Component)]
pub struct SubsectionButton();

#[derive(Component)]
pub struct SidebarItem();

#[derive(Event)]
pub struct SectionVisibilityEvent {
    pub chapter_number: u32,
}

#[derive(Event)]
pub struct SubsectionVisibilityEvent {
    // pub chapter_number: u32,
    pub section_number: u32,
}

// true iff the chapter's sections are shown
#[derive(Component)]
pub struct ShowingSectionsOfThisChapter(bool);

// true iff the section's subsections are shown
#[derive(Component)]
pub struct ShowingSubsectionsOfThisSection(bool);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectionVisibilityEvent>()
            .add_event::<SubsectionVisibilityEvent>()
            .add_systems(Update, (
                section_button_visibility_system,
                subsection_button_visibility_system,
                chapter_button_interaction,
                section_button_interaction,
                subsection_button_interaction,
                sidebar_button_mouse_scroll,
            ));
    }
}

// ================================
// ========== UI Buttons ==========
// ================================

// ---------- Chapter UI Button ----------
pub fn chapter_button(commands: &mut Commands, chapter_name: &String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_button = (
        ChapterButton(),
        SidebarItem(),
        chapter_number,
        ShowingSectionsOfThisChapter(false),
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

// ---------- Section UI Button ----------
pub fn section_button(commands: &mut Commands, chapter_name: &String, chapter_number: u32, section_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let section_number = SectionNumber(section_number);
    let section_button = (
        SectionButton(),
        SidebarItem(),
        section_number,
        chapter_number,
        ShowingSectionsOfThisChapter(false),
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: HIDDEN_SIDEBAR_BUTTON_HEIGHT,
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    );

    let section_button = commands.spawn(section_button).id();

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
    commands.entity(section_button).push_children(&[text_item]);

    return section_button;
}

// ---------- Subsection UI Button ----------
pub fn subsection_button(commands: &mut Commands, chapter_name: &String, chapter_number: u32, section_number: u32, subsection_number: u32) -> Entity {
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
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: HIDDEN_SIDEBAR_BUTTON_HEIGHT,
                border: HIDDEN_BUTTON_BORDER,
                padding: HIDDEN_BUTTON_BORDER,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    );

    let subsection_button = commands.spawn(subsection_button).id();

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
    commands.entity(subsection_button).push_children(&[text_item]);

    return subsection_button;
}

// =========================================
// ========== Interaction Systems ==========
// =========================================

// ---------- Chapter Interaction ----------
fn chapter_button_interaction (
    mut interaction_query: Query<
        (&Interaction, &ChapterNumber, &mut BackgroundColor, &mut BorderColor, &mut ShowingSectionsOfThisChapter),
        (Changed<Interaction>, With<ChapterButton>)
    >,
    mut section_visibility_writer: EventWriter<SectionVisibilityEvent>,
) {
    for (
        interaction,
        chapter_number,
        mut chapter_button_background_color,
        mut chapter_button_border_color ,
        mut showing_sections
    ) in &mut interaction_query {
        
        let mut pressed_color: BackgroundColor = Color::BLACK.into();
        let mut hovered_color: BackgroundColor = Color::BLACK.into();
        match showing_sections.0 {
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
                section_visibility_writer.send(SectionVisibilityEvent{chapter_number: chapter_number.0});
                showing_sections.0 = !showing_sections.0;
            }
            Interaction::Hovered => {
                *chapter_button_background_color = hovered_color;
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
            Interaction::None => {
                *chapter_button_background_color = Color::rgb(0.5, 0.5, 0.5).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}

// ---------- Section Interaction ----------
fn section_button_interaction (
    mut interaction_query: Query<
        (&Interaction, &ChapterNumber, &SectionNumber, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SectionButton>)
    >,
    mut subsection_visibility_writer: EventWriter<SubsectionVisibilityEvent>,
) {
    for (interaction, chapter_number, section_number, mut chapter_button_background_color, mut chapter_button_border_color ) in &mut interaction_query {
        let chapter_number: u32 = chapter_number.0;
        let section_number: u32 = section_number.0;

        match *interaction {
            Interaction::Pressed => {
                println!("Pressed Chapter {}, Section {}", chapter_number, section_number);
                *chapter_button_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                subsection_visibility_writer.send(SubsectionVisibilityEvent{section_number: section_number});
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

// ---------- Subsection Interaction ----------
fn subsection_button_interaction (
    mut interaction_query: Query<
        (&Interaction, &ChapterNumber, &SectionNumber, &SubsectionNumber, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SubsectionButton>)
    >,
    // mut section_visibility_writer: EventWriter<SectionVisibilityEvent>,
) {
    for (interaction, chapter_number, section_number, subsection_number, mut chapter_button_background_color, mut chapter_button_border_color ) in &mut interaction_query {
        let chapter_number: u32 = chapter_number.0;
        let section_number: u32 = section_number.0;
        let subsection_number: u32 = subsection_number.0;

        match *interaction {
            Interaction::Pressed => {
                println!("Pressed Chapter {}, Section {}, Subsection {}",
                    chapter_number,
                    section_number,
                    subsection_number
                );
                *chapter_button_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                *chapter_button_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                // section_visibility_writer.send(SectionVisibilityEvent(chapter_number.0));
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

// =========================================
// ========== Visibility Systems ===========
// =========================================

// ---------- Section Button Visibility ----------
fn section_button_visibility_system (
    mut section_button_query: Query<(&mut Visibility, &mut Style, &mut ShowingSectionsOfThisChapter, &ChapterNumber), With<SectionButton>>,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionButton>>,
    mut section_button_visibility_event: EventReader<SectionVisibilityEvent>,
) {
    for event in section_button_visibility_event.read() {
        for (mut section_button_visibility, mut style, mut showing_sections, section_button_chapter_number) in &mut section_button_query.iter_mut() {
            let chapter_button_chapter_number: u32 = event.chapter_number;
            let section_button_chapter_number: u32 = section_button_chapter_number.0;

            if chapter_button_chapter_number == section_button_chapter_number {
                println!("Pressed Chapter number {}", chapter_button_chapter_number);

                match showing_sections.0 {
                    true => { // Hide section if it's currently shown
                        *section_button_visibility = Visibility::Hidden;
                        style.height = HIDDEN_SIDEBAR_BUTTON_HEIGHT;
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

// ---------- Subsection Button Visibility ----------
fn subsection_button_visibility_system (
    mut subsection_button_query: Query<(&mut Visibility, &mut Style, &mut ShowingSectionsOfThisChapter, &mut ShowingSubsectionsOfThisSection, &ChapterNumber, &SectionNumber, &SubsectionNumber), With<SubsectionButton>>,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionButton>>,
    mut section_button_visibility_event: EventReader<SectionVisibilityEvent>,
    mut subsection_button_visibility_event: EventReader<SubsectionVisibilityEvent>,
) {
    for event in subsection_button_visibility_event.read() {
        for (mut subsection_button_visibility, mut style, mut showing_sections, mut showing_subsections, subsection_button_chapter_number, subsection_button_section_number, subsection_button_subsection_number) in &mut subsection_button_query.iter_mut() {
            let section_button_section_number: u32 = event.section_number;
            let subsection_button_section_number: u32 = subsection_button_section_number.0;

            if section_button_section_number == subsection_button_section_number {
                println!("Pressed Chapter number {}", section_button_section_number);

                match showing_subsections.0 {
                    true => { // Hide section if it's currently shown
                        *subsection_button_visibility = Visibility::Hidden;
                        style.height = HIDDEN_SIDEBAR_BUTTON_HEIGHT;
                        style.border = HIDDEN_BUTTON_BORDER;
                        style.padding = HIDDEN_BUTTON_BORDER;
                    }
                    false => { // show section if it's currently hidden
                        *subsection_button_visibility = Visibility::Inherited;
                        style.height = SIDEBAR_BUTTON_HEIGHT;
                        style.border = SUBSECTION_BUTTON_BORDER;
                        style.padding = SUBSECTION_BUTTON_BORDER;
                    }
                }
                showing_subsections.0 = !showing_subsections.0;
            }
        }
    }
}


// =========================================
// ========== Scrolling Systems ===========
// =========================================
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