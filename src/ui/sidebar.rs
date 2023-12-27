use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use super::scrollable_page;
use super::chapter_container;

// Marker for UI node
#[derive(Component)]
pub struct Sidebar;


#[derive(Component, Default)]
pub struct SidebarList {
    pub position: f32,
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(chapter_container::SystemsPlugin);
        // .add_event::<SidebarScrollEvent>()
        // .add_systems(Update, (sidebar_scroll_reciever));
    }
}

pub fn setup(commands: &mut Commands, width: f32) -> Entity {
    let sidebar = new(width);
    let sidebar = commands.spawn(sidebar).id();

    let page_items = page_items(commands);
    let scrollable_page = scrollable_page::get_page();

    let scrollable_page = commands.spawn((SidebarList::default(), scrollable_page)).id();

    commands.entity(scrollable_page).push_children(&page_items);
    commands.entity(sidebar).push_children(&[scrollable_page]);

    return sidebar;
}

pub fn new(width: f32) -> (Sidebar, ButtonBundle) {
    return (
        Sidebar,
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                width: Val::Percent(width),
                // width: Val::Px(100.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: Color::rgb(1.0, 0.0, 1.0).into(),
            ..default()
        }
    );
}


pub fn page_items(commands: &mut Commands) -> Vec<Entity> {

    // TODO: Add Parts, Chapter, Sections, then subsections:
    // Part 1: Euclidean Space
    //   Chapter 1: Vectors
    //     1.1 What is a vector
    //       1.1.1 List of Components
    //       ...
    //     1.2 Algebra of vectors
    //       ...
    //     ...
    //   Chapter 2: ...
    //     ...
    //   ...
    // Part 2: ...
    //   ...
    let mut page_items = Vec::new();
    // for chapter_number in 0..1000u32 {
    //     let chapter_name = format!("");
    //     let chapter_container = chapter_container::chapter_button(commands, &chapter_name, chapter_number);
        
    //     let section_name1 = format!("Section {}", 1);
    //     let section_name2 = format!("Section {}", 2);
    //     let section_button = chapter_container::section_button(commands, &section_name1, chapter_number, 1);
    //     let section_button2 = chapter_container::section_button(commands, &section_name2, chapter_number, 2);
    //     page_items.push(chapter_container);
    //     page_items.push(section_button);
    //     page_items.push(section_button2);
    // }
    

    // SMALL but important FIXME:
    // split this chapter, section, subsection adding code
    // into functions, but still add them individually here
    // they should be 1 line each instead of 4 bc it would just
    // be a function call :)

    // ============================= //
    // **** PART 1 ***************** //
    // ============================= //
    // ********* Chapter 1 ********* //
    let chapter_number: u32 = 1;
    let chapter_name: String = "Chapter 1. Vectors".into();
    let chapter_button: Entity = chapter_container::chapter_button(commands, &chapter_name, chapter_number);
    page_items.push(chapter_button);
    
    // 1.1
    let section_number: u32 = 1;
    let section_name: String = "1.1 What is a Vector".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "1.1.1 Lists of components".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "1.1.2 Visualizing vectors".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 3;
    let subsection_name: String = "1.1.3 Beyond three dimensions".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 4;
    let subsection_name: String = "1.1.4 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);
    
    // 1.2
    let section_number: u32 = 2;
    let section_name: String = "1.2 Algebra of vectors".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "1.2.1 Adding vectors".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "1.2.2 Scaling vectors".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 3;
    let subsection_name: String = "1.2.3 Linear combinations".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 4;
    let subsection_name: String = "1.2.4 Spans".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 5;
    let subsection_name: String = "1.2.5 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);
    
    // 1.3
    let section_number: u32 = 3;
    let section_name: String = "1.3 Geometry of vectors".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);
    
    let subsection_number: u32 = 1;
    let subsection_name: String = "1.3.1 Geometry in R, R^2, and R^3".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "1.3.2 Dot Product".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 3;
    let subsection_name: String = "1.3.3 Length in R^n".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 4;
    let subsection_name: String = "1.3.4 Angle in R^n".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 5;
    let subsection_name: String = "1.3.5 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    // 1.4
    let section_number: u32 = 4;
    let section_name: String = "1.4 Challenge Problems".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "1.4.1 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);


    // ********* Chapter 2 ********* //
    let chapter_number: u32 = 2;
    let chapter_name: String = "Chapter 2. Subspaces".into();
    let chapter_button: Entity = chapter_container::chapter_button(commands, &chapter_name, chapter_number);
    page_items.push(chapter_button);
    
    // 2.1
    let section_number: u32 = 1;
    let section_name: String = "2.1 Working with subsets".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "2.1.1 Intersections and unions of subsets".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "2.1.2 Sums and translations in R^n (FIXME)".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "2.1.3 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);
    
    // 2.2
    let section_number: u32 = 2;
    let section_name: String = "2.2 What is a subspace?".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "2.2.1 Characterizing subspaces".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "2.2.2 Affine subspaces".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 3;
    let subsection_name: String = "2.2.3 Orthogonal compliments".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 4;
    let subsection_name: String = "2.2.4 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    // 2.3
    let section_number: u32 = 3;
    let section_name: String = "2.3 Efficiently encoding vectors".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "2.3.1 Dimension".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 2;
    let subsection_name: String = "2.3.2 Bases".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 3;
    let subsection_name: String = "2.3.3 Encoding vectors".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 4;
    let subsection_name: String = "2.3.4 Linear independence".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 5;
    let subsection_name: String = "2.3.5 Characterizing bases".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    let subsection_number: u32 = 6;
    let subsection_name: String = "2.3.6 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    // 2.4
    let section_number: u32 = 4;
    let section_name: String = "2.4 Challenge problems".into();
    let section_button: Entity = chapter_container::section_button(commands, &section_name, chapter_number, section_number);
    page_items.push(section_button);

    let subsection_number: u32 = 1;
    let subsection_name: String = "2.4.1 Exercises".into();
    let subsection_button: Entity = chapter_container::subsection_button(commands, &subsection_name, chapter_number, section_number, subsection_number);
    page_items.push(subsection_button);

    // ********* Chapter 3 ********* //

    // ============================= //
    // **** PART 2 ***************** //
    // ============================= //
    // ********* Chapter 4 ********* //
    // ********* Chapter 5 ********* //

    // ============================= //
    // **** PART 3 ***************** //
    // ============================= //
    // ********* Chapter 6 ********* //
    // ********* Chapter 7 ********* //
    // ********* Chapter 8 ********* //

    // ============================= //
    // **** PART 4 ***************** //
    // ============================= //
    // ********* Chapter 9 ********* //
    // ********* Chapter 10 ******** //
    // ********* Chapter 11 ******** //

    // ============================= //
    // **** PART 5 ***************** //
    // ============================= //
    // ********* Chapter 12 ******** //
    // ********* Chapter 13 ******** //
    // ********* Chapter 14 ******** //

    // ============================= //
    // **** BIBLIOGRAPHY *********** //
    // ============================= //



    return page_items;
}
