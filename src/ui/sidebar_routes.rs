use bevy::prelude::*;
use super::chapter_container;
use super::theme;


pub fn page_items(commands: &mut Commands, theme: &theme::CurrentTheme) -> Vec<Entity> {

    let mut page_items = Vec::new();

    // SMALL but important FIXME:
    // split this chapter, section, subsection adding code
    // into functions, but still add them individually here
    // they should be 1 line each instead of 4 bc it would just
    // be a function call :)
    
    page_items.push(chapter_container::header_button(
        commands, theme, &"Chapters".into()));
    
    // ============================= //
    // **** PART 1 ***************** //
    // ============================= //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Home".into(), 0));
    
    // 0.0
    page_items.push(chapter_container::section_button(
        commands,  theme, &"Index".into(), 0, 0));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"Splash Screen".into(), 0, 0, 0));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"Credits".into(), 0, 0, 1));


    page_items.push(chapter_container::title_button(
        commands,  theme, &"Part 1. Euclidean space R^n (FIXME)".into()));
        
    // ********* Chapter 1 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 1. Vectors".into(), 1));
    
    // 1.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"1.1 What is a Vector".into(), 1, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.1.1 Lists of components".into(), 1, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.1.2 Visualizing vectors".into(), 1, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.1.3 Beyond three dimensions".into(), 1, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.1.4 Exercises".into(), 1, 1, 4));

    // 1.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"1.2 Algebra of vectors".into(), 1, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.2.1 Adding vectors".into(), 1, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.2.2 Scaling vectors".into(), 1, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.2.3 Linear combinations".into(), 1, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.2.4 Spans".into(), 1, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.2.5 Exercises".into(), 1, 2, 5));


    
    // 1.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"1.3 Geometry of vectors".into(), 1, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.3.1 Geometry in R, R^2, and R^3".into(), 1, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.3.2 Dot Product".into(), 1, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.3.3 Length in R^n".into(), 1, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.3.4 Angle in R^n".into(), 1, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.3.5 Exercises".into(), 1, 3, 5));
    
    // 1.4
    page_items.push(chapter_container::section_button(
        commands,  theme, &"1.4 Challenge Problems".into(), 1, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"1.4.1 Exercises".into(), 1, 4, 1));



    // ********* Chapter 2 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 2. Subspaces".into(), 2));
    
    // 2.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"2.1 Working with subsets".into(), 2, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.1.1 Intersections and unions of subsets".into(), 2, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.1.2 Sums and translations in R^n (FIXME)".into(), 2, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.1.3 Exercises".into(), 2, 1, 3));

    
    // 2.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"2.2 What is a subspace?".into(), 2, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.2.1 Characterizing subspaces".into(), 2, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.2.2 Affine subspaces".into(), 2, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.2.3 Orthogonal compliments".into(), 2, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.2.4 Exercises".into(), 2, 2, 4));

    // 2.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"2.3 Efficiently encoding vectors".into(), 2, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.1 Dimension".into(), 2, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.2 Bases Affine subspaces".into(), 2, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.3 Encoding vectors".into(), 2, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.4 Linear independence".into(), 2, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.5 Characterizing bases".into(), 2, 3, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.3.6 Exercises".into(), 2, 3, 6));

    // 2.4
    page_items.push(chapter_container::section_button(
        commands,  theme, &"2.4 Challenge problems".into(), 2, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"2.4.1 Exercises".into(), 2, 4, 1));
    

    // FIXME: Tomorrow add all sections and subsections

    // ********* Chapter 3 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 3. Linear functions and transformations".into(), 3));
    
    // 3.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"3.1 Linear functions f : R^n → R (FIXME) ".into(), 3, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.1.1 Multivariable functions".into(), 3, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.1.2 What is a linear function?".into(), 3, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.1.3 Linear functions as networks".into(), 3, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.1.4 \"Dotless\" product with row vector".into(), 3, 1, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.1.5 Exercises".into(), 3, 1, 5));

            
    // 3.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"3.2 Linear transformations T: R^n → R^m (FIXME)".into(), 3, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.1 What is a transformation".into(), 3, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.2 What is a linear transformation?".into(), 3, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.3 Linear transformations as networks".into(), 3, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.4 Visualizing linear transformations".into(), 3, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.5 Characterizing linear transformations".into(), 3, 2, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.2.6 Exercises".into(), 3, 2, 6));

    // 3.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"3.3 Subspaces associated to a linear transformation".into(), 3, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.1 Image of a linear transformation".into(), 3, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.2 Surjectivity".into(), 3, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.3 Null space of a linear transformation".into(), 3, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.4 Pre-image of a general vector".into(), 3, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.5 Injectivity".into(), 3, 3, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.3.6 Exercises".into(), 3, 3, 6));
    
    // 3.4
    page_items.push(chapter_container::section_button(
        commands,  theme, &"3.4 Challenge Problems".into(), 3, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"3.4.1 Exercises".into(), 3, 4, 1));
        
    
    // ============================= //
    // **** PART 2 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands,  theme, &"Part 2. Systems of linear equations".into()));

    // ********* Chapter 4 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 4. Systems of linear equations and their solutions".into(), 4));
    
    // 4.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"4.1 What is a system of linear equations?".into(), 4, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.1.1 Solving a single linear equation".into(), 4, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.1.2 Augmented matrix of a system".into(), 4, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.1.3 Exercises".into(), 4, 1, 3));
    
    // 4.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"4.2 Row echelon form and reduced row echelon form".into(), 4, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.2.1 What is reduced row echelon form?".into(), 4, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.2.2 Solving systems in reduced row echelon form".into(), 4, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.2.3 Exercises".into(), 4, 2, 3));

    // 4.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"4.3 Simplifying systems of linear equations".into(), 4, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.3.1 Row operations".into(), 4, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.3.2 Gaussian Elimination".into(), 4, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.3.3 Solving general systems".into(), 4, 3, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.3.4 Equivalence of systems".into(), 4, 3, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.3.5 Exercises".into(), 4, 3, 5));
    
    // 4.4
    page_items.push(chapter_container::section_button(
        commands,  theme, &"4.4 Challenge Problems".into(), 4, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"4.4.1 Exercises".into(), 4, 4, 1));
            


    // ********* Chapter 5 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 5. Gaussian elimination to the rescue".into(), 5));
    
    // 5.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"5.1 Existence and uniqueness properties?".into(), 5, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.1.1 Is a vector in a subspace?".into(), 5, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.1.2 Does a collection of vectors span in R^m?".into(), 5, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.1.3 Is a collection of vectors linearly independent?".into(), 5, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.1.4 Goldilocks Principle".into(), 5, 1, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.1.5 Exercises".into(), 5, 1, 5));
    

            
    // 5.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"5.2 Basis and dimension computations".into(), 5, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.2.1 Transpose of a matrix".into(), 5, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.2.2 Bases for column and row spaces".into(), 5, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.2.3 Bases for null spaces and orthogonal complements".into(), 5, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.2.4 Rank theorem".into(), 5, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.2.5 Exercises".into(), 5, 2, 5));
    
    
    // 5.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"5.3 Challenge problems".into(), 5, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"5.3.1 Exercises".into(), 5, 3, 1));
                
    

    // ============================= //
    // **** PART 3 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands,  theme, &"Part 3. Matrix algebra".into()));

    // ********* Chapter 6 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 6. Operations on matrices".into(), 6));
    
    // 6.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"6.1 Adding, scaling, and multiplying matrices".into(), 6, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.1 Generalizing vector addition and scaling".into(), 6, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.2 Adding and scaling linear transformations".into(), 6, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.3 Matrix multiplication".into(), 6, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.4 Composing linear transformations".into(), 6, 1, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.5 Matrix multiplications and networks".into(), 6, 1, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.6 Row operations via matrix multiplication".into(), 6, 1, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.1.7 Exercises".into(), 6, 1, 5));
    
    // 6.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"6.2 Matrix transpose".into(), 6, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.2.1 Algebra of matrix transposition".into(), 6, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.2.2 Transpose on networks".into(), 6, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.2.3 Transpose and the dot product".into(), 6, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.2.4 Exercises".into(), 6, 2, 4));
        

    // 6.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"6.3 Challenge problems".into(), 6, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"6.3.1 Exercises".into(), 6, 3, 1));
    
    // ********* Chapter 7 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 7. Operations on matrices".into(), 7));

    // 7.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"7.1 Invertible Matrices".into(), 7, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.1.1 What is the inverse of a matrix?".into(), 7, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.1.2 Finding matrix inverses".into(), 7, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.1.3 Invertible linear transformations".into(), 7, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.1.4 Characterizing invertible matrices".into(), 7, 1, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.1.5 Exercises".into(), 7, 1, 5));

    // 7.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"7.2 Triangular matrices".into(), 7, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.2.1 What is a triangular matrix?".into(), 7, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.2.2 Characterizing upper triangular matrices".into(), 7, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.2.3 Exercises".into(), 7, 2, 3));

    // 7.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"7.3 Orthogonal matrices".into(), 7, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.3.1 What is an orthogonal matrix?".into(), 7, 3, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.3.2 Characterizing orthogonal matrices".into(), 7, 3, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.3.3 Exercises".into(), 7, 3, 3));

    // 7.4
    page_items.push(chapter_container::section_button(
        commands,  theme, &"7.4 Challenge Problems".into(), 7, 4));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"7.4.1 Exercises".into(), 7, 4, 1));
    
    // ********* Chapter 8 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 8. Determinants".into(), 8));
    
    // 8.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"8.1 Determinants of 2 x 2 matrices".into(), 8, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.1.1 Determinants and parallelograms".into(), 8, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.1.2 Areas and linear transformations".into(), 8, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.1.3 Exercises".into(), 8, 1, 3));

            
    // 8.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"8.2 Determinants of general square matrices".into(), 8, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.1 Permutations and general determinants".into(), 8, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.2 Characterizing determinants".into(), 8, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.3 Determinants and Gaussian elimination".into(), 8, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.4 Algebra of determinants".into(), 8, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.5 Reducing determinants to smaller matrices".into(), 8, 2, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.2.6 Exercises".into(), 8, 2, 6));
            
    // 8.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"8.3 Challenge problems".into(), 8, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"8.3.1 Exercises".into(), 8, 3, 1));
    

    // ============================= //
    // **** PART 4 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands,  theme, &"Part 4. Coordinates and Geometry".into()));

    // ********* Chapter 9 ********* //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 9. Coordinates with respect to a basis".into(), 9));
    
    // 9.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"9.1 Linear coordinate systems".into(), 9, 1));
    
        page_items.push(chapter_container::subsection_button(
            commands,  theme, &"9.1.1 Coordinates on a subspace".into(), 9, 1, 1));
        page_items.push(chapter_container::subsection_button(
            commands,  theme, &"9.1.2 Coordinates in R^n (FIXME)".into(), 9, 1, 2));
        page_items.push(chapter_container::subsection_button(
            commands,  theme, &"9.1.3 Exercises".into(), 9, 1, 3));


    // 9.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"9.2 Changing bases".into(), 9, 2));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"9.2.1 Changing coordinates".into(), 9, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"9.2.2 Changing coordinates in R^n (FIXME)".into(), 9, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"9.2.3 Matrix algebra of coordinate changes".into(), 9, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"9.2.4 Exercises".into(), 9, 2, 4));
    
    

    // ********* Chapter 10 ******** //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 10. Orthogonal bases".into(), 10));

    // 10.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"10.1 Coordinates with respect to orthogonal bases".into(), 10, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.1.1 A formula for coordinates".into(), 10, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.1.2 Orthonormal change of bases".into(), 10, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.1.3 Exercises".into(), 10, 1, 3));

    // 10.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"10.2 Constructing orthogonal bases".into(), 10, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.2.1 Gram-Schmidt process".into(), 10, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.2.2 QR factorization (FIXME)".into(), 10, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.2.3 Exercises".into(), 10, 2, 3));
    
    // 10.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"10.3 Challenge problems".into(), 10, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"10.3.1 Exercises".into(), 10, 3, 1));
    
    // ********* Chapter 11 ******** //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 11. Orthogonal projections".into(), 11));
    
    // 11.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"11.1 What is a projection?".into(), 11, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.1.1 Complementary subspaces".into(), 11, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.1.2 Projections and orthogonal projections".into(), 11, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.1.3 Characterizing projections".into(), 11, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.1.4 Exercises".into(), 11, 1, 4));
    
    // 11.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"11.2 Geometry of orthogonal projections".into(), 11, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.1 Minimal distance to a subspace".into(), 11, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.2 Geometry of Gram-Schmidt".into(), 11, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.3 Inconsistent systems and least-squares solutions".into(), 11, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.4 Least-square solutions and normal equations".into(), 11, 2, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.5 Unique least-squares solutions".into(), 11, 2, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"11.2.6 Exercises".into(), 11, 2, 6));


    // 11.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"11.3 Challenge problems".into(), 11, 3));

        page_items.push(chapter_container::subsection_button(
            commands,  theme, &"11.3.1 Exercises".into(), 11, 3, 1));
    
    // ============================= //
    // **** PART 5 ***************** //
    // ============================= //
    page_items.push(chapter_container::title_button(
        commands,  theme, &"Part 5. Choose your friends and your bases wisely".into()));

    // ********* Chapter 12 ******** //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 12. Linear transformations and coordinates".into(), 12));
    
    // 12.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"12.1 Linear transformations and coordinates".into(), 12, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.1.1 Turning linear transformations into matrices".into(), 12, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.1.2 Exercises".into(), 12, 1, 2));
    
    // 12.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"12.2 Simplifying linear transformations".into(), 12, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.2.1 Rank is everything".into(), 12, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.2.2 Similar matrices".into(), 12, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.2.3 Diagonalizability".into(), 12, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.2.4 Exercises".into(), 12, 2, 4));
    
    // 12.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"12.3 Problems".into(), 12, 3));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"12.3.1 Exercises".into(), 12, 3, 1));
        
    // ********* Chapter 13 ******** //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 13. Eigenvalues and eigenvectors".into(), 13));
    
    // 13.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"13.1 Eigenvalues, eigenvectors, and eigenspaces".into(), 13, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.1 What are eigenvalues and eigenvectors?".into(), 13, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.2 Finding eigenvalues".into(), 13, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.3 Finding eigenvectors".into(), 13, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.4 Interlude on complex numbers".into(), 13, 1, 4));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.5 Complex eigenvalues, eigenvectors, and diagonalizability".into(), 13, 1, 5));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.1.6 Exercises".into(), 13, 1, 6));
        

    // 13.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"13.2 Diagonalizing Matrices".into(), 13, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.2.1 Dimension of Eigenspaces".into(), 13, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.2.2 Different eigenspaces".into(), 13, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.2.3 When is a matrix diagonalizable?".into(), 13, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.2.4 Exercises".into(), 13, 2, 4));
    
    // 13.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"13.3 Challenge Problems".into(), 13, 3));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"13.3.1 Exercises".into(), 13, 3, 1));

    // ********* Chapter 14 ******** //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Chapter 14. The Spectral Theorem".into(), 14));
    
    // 14.1
    page_items.push(chapter_container::section_button(
        commands,  theme, &"14.1 Symmetric Matrices".into(), 14, 1));
    
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.1.1 What is a symmetric matrix".into(), 14, 1, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.1.2 Characterizing symmetric matrices".into(), 14, 1, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.1.3 Orthogonal diagonalizability".into(), 14, 1, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.1.4 Exercises".into(), 14, 1, 4));
        
    // 14.2
    page_items.push(chapter_container::section_button(
        commands,  theme, &"14.2 Singular Value Decompositions".into(), 14, 2));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.2.1 What is a singular value decomposition?".into(), 14, 2, 1));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.2.2 Geometry of a singular value decomposition".into(), 14, 2, 2));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.2.3 Low rank approximations".into(), 14, 2, 3));
    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.2.4 Exercises".into(), 14, 2, 4));
    // page_items.push(chapter_container::subsection_button(
    //     commands,  theme, &"14.2.5 Exercises".into(), 14, 2, 5));        
    
    // 14.3
    page_items.push(chapter_container::section_button(
        commands,  theme, &"14.3 Challenge Problems".into(), 14, 3));

    page_items.push(chapter_container::subsection_button(
        commands,  theme, &"14.3.1 Exercises".into(), 14, 3, 1));
    
    // ============================= //
    // **** BIBLIOGRAPHY *********** //
    // ============================= //
    page_items.push(chapter_container::chapter_button(
        commands,  theme, &"Bibliography".into(), 15));



    return page_items;
}