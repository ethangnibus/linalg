use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

use super::super::components::{
    page_header,
    text_section,
    definition_block,
    definition_text_section,
    span_of_vectors_renderer,
    example_header,
    solution_header,
};
use super::super::subsection_cameras;
use super::super::theme;



pub fn get(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    asset_server: & Res<AssetServer>,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {
    let next_item = page_header::spawn(commands, theme, "1.2.4 Spans");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   When we have a group of vectors, \
        it's useful to figure out all the places \
        we can go by following those vectors. We \
        call this reachable space the \"span\" of \
        our group of vectors. In this chapter, we \
        will walk through why finding where vectors \
        could reach is important."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   For example, if we have a spaceship in \
        the following 2D world, which planets would we \
        be able to visit? You could drag the vectors \
        coming out of the spaceship to try for yourself \
        in Example 1."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = example_header::spawn(commands, theme, "Example 1");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        0,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);
    

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Evidently, we are able to reach all planets! \
        Notice that here we are using two vectors that point \
        in different directions. Since these vectors point in \
        different directions, they enable us to travel across \
        all 2-dimensional space. Therefore, we can reach every \
        planet in our 2-dimensional solar system."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);
    
    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Would we still be able to travel across all \
        all 2-dimensional space if our spaceship could only \
        move in one direction? Try for yourself in Example \
        2:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = example_header::spawn(commands, theme, "Example 2");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        1,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Unfortunately, our spaceship is now only \
        able to reach three planets. With one vector, our \
        spaceship is restricted to moving along a single line \
        that cuts through our 2-dimensional space. Additionally, all \
        planets that we can visit fall on this line at some point."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   This is one reason why span is important. \
        If we want to build a spaceship that could reach every planet \
        in our solar system, then we must choose a valid set of \
        movement vectors that can reach all of 2-dimensional space. \
        Having only one movement vector would be an invalid set \
        because we can only move along a line with it. In other words, \
        this means we can only move across a 1-dimensional \
        subsection of 2-dimensional space."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   It seems that the set of vectors that we have \
        and where we can reach in our 2-dimensional space \
        are related. Let's walk through how the properties of \
        our set of vectors can change where we can reach in our \
        2-dimensional space. To do this, let's consider every case \
        our set of vectors can appear and visualize it's span."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Case 1: Our set of vectors contains no vectors");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   In this case, we have a set of vectors that has no \
        vectors in it. We can write this as an empty set {} or as a \
        the null vector {0}. To visualize this set of vectors span, \
        try visiting planets in Example 3 by moving along the null \
        vector. Our spaceship will drop space dust to visualize span \
        as it moves through the 2-dimensional space."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 3");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        2,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Interesting. With a set of no vectors, we can still visit one planet. \
        our spaceship drops space dust on the planet we start at. Here, we could say \
        that our spacship spans the null vector."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);










    let next_item = example_header::spawn(commands, theme, "Case 2: Our set of vectors contains one non-null vector");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Here we are revisiting the case where we have one vector \
        that points in a directon other than zero. Which planets can we \
        reach with this set? This time, when you drag the spaceship, It \
        will emit spacedust to mark where it knows it can travel to. Try \
        it yourself in Example 4:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 4");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        3,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Great, our spaceship can move along a line \
        and visit three planets just like before! This time \
        we can visually see the span of our set of vectors... \
        it's shown by the trail of space dust we left behind. \
        Notice that this trail also includes the zero vector. \
        This is because if we don't along our vector, it's the \
        same as moving along the null vector. \
        In Example 5 you can see what this span would look like if we \
        extended our vectors out into infinity:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = example_header::spawn(commands, theme, "Example 5");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        5,
    );




    let next_item = example_header::spawn(commands, theme, "Case 3: Our set of vectors contains two vectors that point in different directions");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
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
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 6");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        6,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Notice that if you move the spaceship as far as we \
        allow you to, the span will cover all of our 2-dimensional space. \
        Since the span covers all of 2-dimensional space this means that \
        our spaceship can reach every planet!"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);




    let next_item = example_header::spawn(commands, theme, "Case 4: Our set of vectors contains two vectors that point the same direction");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   What if both of the vectors in our set pointed in the same direction. \
        Would we still be able to reach all of the planets in our solar system? Try
        dragging the spaceship in Example 7 to find out."
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 7");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        7,
    );

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
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
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "   Let's recap our findings by explaining how many linearly
        independent vectors we would need to span spaces with different dimensions:
        "
    );
    commands.entity(view_list_entity).push_children(&[next_item]);
    

    let next_item = example_header::spawn(commands, theme, "RECAP");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "With zero linear independent vectors, \
        we can only span zero dimensional space. See Example 8:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = example_header::spawn(commands, theme, "Example 8");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        8,
    );



    let next_item = text_section::spawn(
        commands,
        theme, 
        "With one linear independent vector, \
        we can span 1 dimensional space. See Example 9:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 9");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        9,
    );


    let next_item = text_section::spawn(
        commands,
        theme, 
        "With two linear independent vector, \
        we can span 2 dimensional space. See Example 10:"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);


    let next_item = example_header::spawn(commands, theme, "Example 10");
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        10,
    );


    let next_item = example_header::spawn(commands, theme, "Test Yourself:");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "Adding what type of vectors to your set will increase the span of you set of vectors.\n
        a) Linearly independent\n
        b) Linearly dependent\n
        c) Vectors that point in the same direction as other vectors in your set
        d) Vectors that point in different directions that other vector in your set, so long as that different direction isn't a scalar multiple of a direction that already exists in the set"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(
        commands,
        theme, 
        "Adding what type of vectors to your set will increase the span of you set of vectors.\n
        a) Correct. Linearly independent\n
        b) Incorrect. Linearly dependent\n
        c) Incorrect. Vectors that point in the same direction as other vectors in your set\n
        d) Correct. (Explain here)\n\n
        ** Remember to add a mutiple chioce bubble minigame to do stuff like this eventually**"
    );
    commands.entity(view_list_entity).push_children(&[next_item]);





    let span_of_vectors_left = definition_text_section::spawn(commands, "Given a collection of vectors v1, . . . , vk ∈ Rn, their span\nSpan {v1,...,vk} ⊂ Rn\nis the set of all their linear combinations. In other words, Span {v1,...,vk} consists of all v ∈ Rn that can be expressed in the form\nv=a1v1 +···+akvk\nfor some weights a1,...,ak ∈ R.\nGeometrically, the span of a collection of vectors is the set of all vectors that can be reached by trav- eling along scales of each of the individual vectors in turn.");
    let span_of_vectors_right = span_of_vectors_renderer::spawn(commands);
    let next_item = definition_block::spawn(commands, "Span of vectors", span_of_vectors_left, span_of_vectors_right);
    commands.entity(view_list_entity).push_children(&[next_item]);

    subsection_cameras::setup_camera(
        commands,
        theme,
        camera_setup_writer,
        &"3.png".into(),
        5.5, Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        4,
    );

    let next_item = example_header::spawn(commands, theme, "Example 16");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(commands, theme, "Consider the vectors
    1 −1 v1=1,v2= 2
    Give a geometric description of their span Span {v1,v2}.");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = solution_header::spawn(commands, theme, "SOLUTION");
    commands.entity(view_list_entity).push_children(&[next_item]);

    let next_item = text_section::spawn(commands, theme, "Let’s think about the set inside R2 of all linear
    combinations of our vectors 1 −1
    a1v1+a2v2=a1 1 +a2 2 foranya1,a2∈R
    To understand what this set looks like, we will build up the linear combinations
    in several steps.
    • Setting a2 = 0, we get all scales of the first vector 1
    a1v1 =a1 1 foranya1 ∈R
    This gives Span {v1}, the straight line through the origin containing v1.
    • Similarly, setting a1 = 0, we get all scales of the second vector −1
    a2v2 =a2 2 foranya2 ∈R
    This gives Span {v2}, the straight line through the origin containing v2.
    • Finally observe that Span {v1,v2} consists of all vectors formed by adding
    vectors along these
    two straight lines. Since any vector in the plane can be reached in this way,
    we conclude that Span {v1, v2} = R2");
    commands.entity(view_list_entity).push_children(&[next_item]);

    // page_entities.push(
    //     definition_body::spawn(commands, "Span of vectors", content)
    // );

    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    let next_item = page_header::spawn(commands, theme, "Filler just to test :)");
    commands.entity(view_list_entity).push_children(&[next_item]);
    
    
    
}

// add a system to look at all svg_text blocks above
// make a thing called an svg_text block that will hold the math text svg's we use for
// everything
// in the update system, when this page is loaded, send an event to load the svgs and put them in
// the svg blocks.
// we know the page is loaded when the title block appears. Query the title block and get
// the resources based on it's name (or make a struct that tells us which chapter this is)
// use that name to get the svgs.
