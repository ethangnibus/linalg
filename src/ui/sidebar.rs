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
                // width: Val::Px(500.0),
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
    
    page_items.push(chapter_container::header_button(
        commands, &"Chapters".into()));
    
    // ============================= //
    // **** PART 1 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands, &"Part 1. Euclidean space R^n (FIXME)".into()));
        
    // ********* Chapter 1 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 1. Vectors".into(), 1));
    
    // 1.1
    page_items.push(chapter_container::section_button(
        commands, &"1.1 What is a Vector".into(), 1, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"1.1.1 Lists of components".into(), 1, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.1.2 Visualizing vectors".into(), 1, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.1.3 Beyond three dimensions".into(), 1, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.1.4 Exercises".into(), 1, 1, 4));

    // 1.2
    page_items.push(chapter_container::section_button(
        commands, &"1.2 Algebra of vectors".into(), 1, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"1.2.1 Adding vectors".into(), 1, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.2.2 Scaling vectors".into(), 1, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.2.3 Linear combinations".into(), 1, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.2.4 Spans".into(), 1, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.2.5 Exercises".into(), 1, 2, 5));


    
    // 1.3
    page_items.push(chapter_container::section_button(
        commands, &"1.3 Geometry of vectors".into(), 1, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"1.3.1 Geometry in R, R^2, and R^3".into(), 1, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.3.2 Dot Product".into(), 1, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.3.3 Length in R^n".into(), 1, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.3.4 Angle in R^n".into(), 1, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands, &"1.3.5 Exercises".into(), 1, 3, 5));
    
    // 1.4
    page_items.push(chapter_container::section_button(
        commands, &"1.4 Challenge Problems".into(), 1, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"1.4.1 Exercises".into(), 1, 4, 1));



    // ********* Chapter 2 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 2. Subspaces".into(), 2));
    
    // 2.1
    page_items.push(chapter_container::section_button(
        commands, &"2.1 Working with subsets".into(), 2, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"2.1.1 Intersections and unions of subsets".into(), 2, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.1.2 Sums and translations in R^n (FIXME)".into(), 2, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.1.3 Exercises".into(), 2, 1, 3));

    
    // 2.2
    page_items.push(chapter_container::section_button(
        commands, &"2.2 What is a subspace?".into(), 2, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"2.2.1 Characterizing subspaces".into(), 2, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.2.2 Affine subspaces".into(), 2, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.2.3 Orthogonal compliments".into(), 2, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.2.4 Exercises".into(), 2, 2, 4));

    // 2.3
    page_items.push(chapter_container::section_button(
        commands, &"2.3 Efficiently encoding vectors".into(), 2, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.1 Dimension".into(), 2, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.2 Bases Affine subspaces".into(), 2, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.3 Encoding vectors".into(), 2, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.4 Linear independence".into(), 2, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.5 Characterizing bases".into(), 2, 3, 5));
    page_items.push(chapter_container::subsection_button(
        commands, &"2.3.6 Exercises".into(), 2, 3, 6));

    // 2.4
    page_items.push(chapter_container::section_button(
        commands, &"2.4 Challenge problems".into(), 2, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands, &"2.4.1 Exercises".into(), 2, 4, 1));
    

    // FIXME: Tomorrow add all sections and subsections

    // ********* Chapter 3 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 3. Linear functions and transformations".into(), 3));
    
    // 3.1
    page_items.push(chapter_container::section_button(
        commands, &"3.1 Linear functions f : R^n → R (FIXME) ".into(), 3, 1));
    
    // 3.2
    page_items.push(chapter_container::section_button(
        commands, &"3.2 Linear transformations T: R^n → R^m (FIXME)".into(), 3, 2));

    // 3.3
    page_items.push(chapter_container::section_button(
        commands, &"3.3 Subspaces associated to a linear transformation".into(), 3, 3));
    
    // 3.4
    page_items.push(chapter_container::section_button(
        commands, &"3.4 Challenge Problems".into(), 3, 4));
    
    // ============================= //
    // **** PART 2 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands, &"Part 2. Systems of linear equations".into()));

    // ********* Chapter 4 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 4. Systems of linear equations and their solutions".into(), 4));
    
    // 4.1
    page_items.push(chapter_container::section_button(
        commands, &"4.1 What is a system of linear equations?".into(), 4, 1));
    
    // 4.2
    page_items.push(chapter_container::section_button(
        commands, &"4.2 Row echelon form and reduced row echelon form".into(), 4, 2));
    
    // 4.3
    page_items.push(chapter_container::section_button(
        commands, &"4.3 Simplifying systems of linear equations".into(), 4, 3));
    
    // 4.4
    page_items.push(chapter_container::section_button(
        commands, &"4.4 Challenge Problems".into(), 4, 4));
    


    // ********* Chapter 5 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 5. Gaussian elimination to the rescue".into(), 5));
    
    // 5.1
    page_items.push(chapter_container::section_button(
        commands, &"5.1 Existence and uniqueness properties?".into(), 5, 1));
    
    // 5.2
    page_items.push(chapter_container::section_button(
        commands, &"5.2 Basis and dimension computations".into(), 5, 2));
    
    // 5.3
    page_items.push(chapter_container::section_button(
        commands, &"5.3 Challenge problems".into(), 5, 3));
    

    // ============================= //
    // **** PART 3 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands, &"Part 3. Matrix algebra".into()));

    // ********* Chapter 6 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 6. Operations on matrices".into(), 6));
    
    // 6.1
    page_items.push(chapter_container::section_button(
        commands, &"6.1 Adding, scaling, and multiplying matrices".into(), 6, 1));
    
    // 6.2
    page_items.push(chapter_container::section_button(
        commands, &"6.2 Matrix transpose".into(), 6, 2));
    
    // 6.3
    page_items.push(chapter_container::section_button(
        commands, &"6.3 Challenge problems".into(), 6, 3));
    

    // ********* Chapter 7 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 7. Operations on matrices".into(), 7));

    // 7.1
    page_items.push(chapter_container::section_button(
        commands, &"7.1 Invertible Matrices".into(), 7, 1));
    
    // 7.2
    page_items.push(chapter_container::section_button(
        commands, &"7.2 Triangular matrices".into(), 7, 2));
    
    // 7.3
    page_items.push(chapter_container::section_button(
        commands, &"7.3 Orthogonal matrices".into(), 7, 3));
    
    // 7.4
    page_items.push(chapter_container::section_button(
        commands, &"7.4 Challenge Problems".into(), 7, 4));
        
    // ********* Chapter 8 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 8. Determinants".into(), 8));
    
    // 8.1
    page_items.push(chapter_container::section_button(
        commands, &"8.1 Determinants of 2 x 2 matrices".into(), 8, 1));
    
    // 8.2
    page_items.push(chapter_container::section_button(
        commands, &"8.2 Determinants of general square matrices".into(), 8, 2));

    // 8.3
    page_items.push(chapter_container::section_button(
        commands, &"8.3 Challenge problems".into(), 8, 3));
    

    // ============================= //
    // **** PART 4 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands, &"Part 4. Coordinates and Geometry".into()));

    // ********* Chapter 9 ********* //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 9. Coordinates with respect to a basis".into(), 9));
    
    // 9.1
    page_items.push(chapter_container::section_button(
        commands, &"9.1 Linear coordinate systems".into(), 9, 1));
    
    // 9.2
    page_items.push(chapter_container::section_button(
        commands, &"9.2 Changing bases".into(), 9, 2));

    

    // ********* Chapter 10 ******** //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 10. Orthogonal bases".into(), 10));

    // 10.1
    page_items.push(chapter_container::section_button(
        commands, &"10.1 Coordinates with respect to orthogonal bases".into(), 10, 1));
    
    // 10.2
    page_items.push(chapter_container::section_button(
        commands, &"10.2 Constructing orthogonal bases".into(), 10, 2));

    // 10.3
    page_items.push(chapter_container::section_button(
        commands, &"10.3 Challenge problems".into(), 10, 3));

    // ********* Chapter 11 ******** //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 11. Orthogonal projections".into(), 11));
    
    // 11.1
    page_items.push(chapter_container::section_button(
        commands, &"11.1 What is a projection?".into(), 11, 1));
    
    // 11.2
    page_items.push(chapter_container::section_button(
        commands, &"11.2 Geometry of orthogonal projections".into(), 11, 2));

    // 11.3
    page_items.push(chapter_container::section_button(
        commands, &"11.3 Challenge problems".into(), 11, 3));

    // ============================= //
    // **** PART 5 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands, &"Part 5. Choose your friends and your bases wisely".into()));

    // ********* Chapter 12 ******** //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 12. Linear transformations and coordinates".into(), 12));
    
    // 12.1
    page_items.push(chapter_container::section_button(
        commands, &"12.1 Linear transformations and coordinates".into(), 12, 1));
    
    // 12.2
    page_items.push(chapter_container::section_button(
        commands, &"12.2 Simplifying linear transformations".into(), 12, 2));

    // 12.3
    page_items.push(chapter_container::section_button(
        commands, &"12.3 Problems".into(), 12, 3));

    // ********* Chapter 13 ******** //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 13. Eigenvalues and eigenvectors".into(), 13));
    
    // 13.1
    page_items.push(chapter_container::section_button(
        commands, &"13.1 Eigenvalues, eigenvectors, and eigenspaces".into(), 13, 1));
    
    // 13.2
    page_items.push(chapter_container::section_button(
        commands, &"13.2 Diagonalizing Matrices".into(), 13, 2));

    // 13.3
    page_items.push(chapter_container::section_button(
        commands, &"13.3 Challenge Problems".into(), 13, 3));

    // ********* Chapter 14 ******** //
    page_items.push(chapter_container::chapter_button(
        commands, &"Chapter 14. The Spectral Theorem".into(), 14));
    
    // 14.1
    page_items.push(chapter_container::section_button(
        commands, &"14.1 Symmetric Matrices".into(), 14, 1));
    
    // 14.2
    page_items.push(chapter_container::section_button(
        commands, &"14.2 Singular Value Decompositions".into(), 14, 2));

    // 14.3
    page_items.push(chapter_container::section_button(
        commands, &"14.3 Challenge Problems".into(), 14, 3));

    // ============================= //
    // **** BIBLIOGRAPHY *********** //
    // ============================= //
    page_items.push(chapter_container::chapter_button(
        commands, &"Bibliography".into(), 15));



    return page_items;
}