use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};

use crate::ui::components::{
    page_header,
    text_section,
    definition_block,
    definition_text_section,
    span_of_vectors_renderer,
    example_header,
    solution_header,
    sub_header,
    example_block,
};
use crate::ui::subsection_cameras;
use crate::ui::util::theme;
use super::{
    example_0,
    example_1,
};



pub fn get(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut asset_server: &mut Res<AssetServer>,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {
    // subsection_cameras::setup_light(commands, film_crew_entity, -10.0, 5.0, 10.0);
    page_header::spawn(commands, theme, view_list_entity, "1.2.4 Spans");

    text_section::spawn(
        commands,
        theme, 
        view_list_entity,
        "   When we have a set of vectors, \
        it's useful to know every location we can \
        visit by following those vectors. We \
        call the set of locations we can visit the \"span\" of \
        our group of vectors. In this chapter, we \
        will walk through why finding where vectors \
        could reach is important.\n\n\
        Let's begin with a motivated example. \
        Say we have a spaceship in the following 3D solar system. \
        The spaceship's engineers gave it a set of vectors that \
        allows it to reach every location in the solar system. \
        To find out how, we attached a device to our spaceship \
        that allows us to see all the places that our spaceship \
        could reach. Try visiting all the suns in the \
        Example 0's solar system by translating the spaceship by it's vectors. \
        As you do so, notice how the shape that trails behind \
        our spaceship changes. This reveals the span \
        of our set of vectors.
        "
    );

    solution_header::spawn(commands, theme, view_list_entity, "DIRECTIONS");

    text_section::spawn(
        commands,
        theme, 
        view_list_entity,
        "- To enter the example, press the button on the bottom right of \
        the viewport below.\n\n\
        - You can zoom by scrolling, orbit the camera by clicking and \
        dragging the background with left mouse, and pan by using the \
        clicking and dragging with the right mouse.\n\n\
        - You can click on the spaceship to toggle the visibility of it's \
        vectors. Click and drag on a vector to translate the spaceship by \
        the vector.
        "
    );

    example_block::spawn(
        commands,
        theme,
        film_crew_entity,
        Val::Px(500.0),
        meshes,
        materials,
        images,
        asset_server,
        view_list_entity,
        0,
    );
    example_0::setup_scene(
        commands,
        theme,
        film_crew_entity,
        meshes,
        materials,
        asset_server,
        0,
    );
    let space_under = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
            style: Style {
                height: Val::Px(20.0),
                width: Val::Px(100.0),
                ..default()
            },
            ..default()
        }
    )).id();

    commands.entity(view_list_entity).push_children(&[space_under]);

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");
    

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   In reality the span extends infinitely in the direction \
        of each vector in our set. In this case it would cover all of 3D space. \
        We chose to show a subset of the span so you could \
        see how the span is constructed by every location our \
        spaceship can reach as we translate its vectors to infinity.\n\n\
        You might be asking why we can reach all of 3 dimensional space with the \
        set of vectors that we chose. To investigate this further, let's \
        see where we can reach when we restrict our set of vectors. \
        Rather than using a spaceship as an example, let's use a sphere.
        "
    );

    sub_header::spawn(commands, theme, view_list_entity, "Case 1: Our set of vectors contains no vectors");
    
    
    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   With no vectors in our set, we have no vectors to \
        translate our sphere by. Therefore, the sphere is stuck at \
        its starting position. In our 3D world, this is given by \
        [0.0, 0.0, 0.0]. Commonly we refer to this as a null vector. \
        Try moving the sphere in Example 1:"
    );

    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     asset_server,
    //     view_list_entity,
    //     1,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     asset_server,
    //     1,
    // );
    let space_under = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
            style: Style {
                height: Val::Px(20.0),
                width: Val::Px(100.0),
                ..default()
            },
            ..default()
        }
    )).id();

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Unfortunately, our spaceship is now only \
        able to reach three planets. With one vector, our \
        spaceship is restricted to moving along a single line \
        that cuts through our 2-dimensional space. Additionally, all \
        planets that we can visit fall on this line at some point."
    );

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   This is one reason why span is important. \
        If we want to build a spaceship that could reach every planet \
        in our solar system, then we must choose a valid set of \
        movement vectors that can reach all of 2-dimensional space. \
        Having only one movement vector would be an invalid set \
        because we can only move along a line with it. In other words, \
        this means we can only move across a 1-dimensional \
        subsection of 2-dimensional space."
    );

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   It seems that the set of vectors that we have \
        and where we can reach in our 2-dimensional space \
        are related. Let's walk through how the properties of \
        our set of vectors can change where we can reach in our \
        2-dimensional space. To do this, let's consider every case \
        our set of vectors can appear and visualize it's span."
    );


    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   In this case, we have a set of vectors that has no \
        vectors in it. We can write this as an empty set {} or as a \
        the null vector {0}. To visualize this set of vectors span, \
        try visiting planets in Example 3 by moving along the null \
        vector. Our spaceship will drop space dust to visualize span \
        as it moves through the 2-dimensional space."
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     3,
    // );

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Interesting. With a set of no vectors, we can still visit one planet. \
        our spaceship drops space dust on the planet we start at. Here, we could say \
        that our spaceship spans the null vector."
    );










    sub_header::spawn(commands, theme, view_list_entity, "Case 2: Our set of vectors contains one non-null vector");
    

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Here we are revisiting the case where we have one vector \
        that points in a directon other than zero. Which planets can we \
        reach with this set? This time, when you drag the spaceship, It \
        will emit spacedust to mark where it knows it can travel to. Try \
        it yourself in Example 4:"
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     4,
    // );

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Great, our spaceship can move along a line \
        and visit three planets just like before! This time \
        we can visually see the span of our set of vectors... \
        it's shown by the trail of space dust we left behind. \
        Notice that this trail also includes the zero vector. \
        This is because if we don't move along our vector, it's the \
        same as moving along the null vector. \
        In Example 5 you can see what this span would look like if we \
        extended our vectors out into infinity:"
    );

    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     5,
    // );




    sub_header::spawn(commands, theme, view_list_entity, "Case 3: Our set of vectors contains two vectors that point in different directions");
    

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Let's look back to when we had two vectors that pointed \
        in different directions. Since these vectors point in different \
        directions, we call them linearly independent. In other words, the \
        line that you travel along when you move along one vector is different \
        than the line you move along when you travel along the other vector. Try \
        visiting planets again by dragging vectors. This time when you drag a vector, \
        the 1-dimensional span of that vector will appear as a thin line extending to infinity. \
        When the spaceship moves, it still leave a trail of space dust to visualize the \
        span of vectors. Notice that if you drag in one direction the span will \
        be limited to a 1-dimensional line. Once you move in the second direction, \
        the span will start to cover two dimensions, or a 2-dimensional plane! \
        Try it below in Example 6:"
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     6,
    // );

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Notice that if you move the spaceship as far as we \
        allow you to, the span will cover all of our 2-dimensional space. \
        Since the span covers all of 2-dimensional space this means that \
        our spaceship can reach every planet!"
    );




    sub_header::spawn(commands, theme, view_list_entity, "Case 4: Our set of vectors contains two vectors that point the same direction");
    

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   What if both of the vectors in our set pointed in the same direction. \
        Would we still be able to reach all of the planets in our solar system? Try
        dragging the spaceship in Example 7 to find out."
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     7,
    // );

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Now our spaceship fails to reach all planets. \
        Since both vectors point in the same direction, the 1-dimensional \
        line that they let us move along are the same. In other words, \
        these vectors are colinear. We could add as many colinear vectors \
        as we wanted to our set and we would still be restriced to moving \
        along a line. If we want to enable our ship to visit every planet, \
        we would have to add a vector to our set that isn't colinear. In other \
        words, we would have to add a vector that points in a different direction \
        than the rest."
    );

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "   Let's recap our findings by explaining how many linearly
        independent vectors we would need to span spaces with different dimensions:
        "
    );
    

    page_header::spawn(commands, theme, view_list_entity, "RECAP");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "With zero linear independent vectors, \
        we can only span zero dimensional space. See Example 8:"
    );

    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     8,
    // );



    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "With one linear independent vector, \
        we can span 1 dimensional space. See Example 9:"
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     9,
    // );


    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "With two linear independent vector, \
        we can span 2 dimensional space. See Example 10:"
    );


    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     10,
    // );


    sub_header::spawn(commands, theme, view_list_entity, "Test Yourself:");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "Adding what type of vectors to your set will increase the span of you set of vectors.\n
        a) Linearly independent\n
        b) Linearly dependent\n
        c) Vectors that point in the same direction as other vectors in your set
        d) Vectors that point in different directions that other vector in your set, so long as that different direction isn't a scalar multiple of a direction that already exists in the set"
    );

    solution_header::spawn(commands, theme, view_list_entity, "SOLUTION");

    text_section::spawn(
        commands,
        theme,
        view_list_entity,
        "Adding what type of vectors to your set will increase the span of you set of vectors.\n
        a) Correct. Linearly independent\n
        b) Incorrect. Linearly dependent\n
        c) Incorrect. Vectors that point in the same direction as other vectors in your set\n
        d) Correct. (Explain here)\n\n
        ** Remember to add a mutiple chioce bubble minigame to do stuff like this eventually**"
    );





    let span_of_vectors_left = definition_text_section::spawn(commands, "Given a collection of vectors v1, . . . , vk ∈ Rn, their span\nSpan {v1,...,vk} ⊂ Rn\nis the set of all their linear combinations. In other words, Span {v1,...,vk} consists of all v ∈ Rn that can be expressed in the form\nv=a1v1 +···+akvk\nfor some weights a1,...,ak ∈ R.\nGeometrically, the span of a collection of vectors is the set of all vectors that can be reached by trav- eling along scales of each of the individual vectors in turn.");
    let span_of_vectors_right = span_of_vectors_renderer::spawn(commands);
    let next_item = definition_block::spawn(commands, theme, "Span of vectors", span_of_vectors_left, span_of_vectors_right);
    commands.entity(view_list_entity).push_children(&[next_item]);

    // example_block::spawn(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     Val::Px(500.0),
    //     meshes,
    //     materials,
    //     images,
    //     view_list_entity,
    //     15,
    // );






    // FILLER

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");

    page_header::spawn(commands, theme, view_list_entity, "Filler just to test :)");


    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     2,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     3,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     4,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     5,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     6,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     7,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     8,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     9,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     10,
    // );
    // example_1::setup_scene(
    //     commands,
    //     theme,
    //     film_crew_entity,
    //     meshes,
    //     materials,
    //     15,
    // );

}

// add a system to look at all svg_text blocks above
// make a thing called an svg_text block that will hold the math text svg's we use for
// everything
// in the update system, when this page is loaded, send an event to load the svgs and put them in
// the svg blocks.
// we know the page is loaded when the title block appears. Query the title block and get
// the resources based on it's name (or make a struct that tells us which chapter this is)
// use that name to get the svgs.
