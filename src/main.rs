#![allow(unused_imports)]
use taffy::{
    print_tree,
    style_helpers::{auto, fr, TaffyAuto, TaffyGridLine, TaffyGridSpan},
    AvailableSpace, Dimension, Display, GridPlacement, JustifyContent, Line, MinMax,
    MinTrackSizingFunction, Size, Style, TaffyTree, TrackSizingFunction,
};

fn line(index: i16, span: u16) -> Line<GridPlacement> {
    Line {
        start: GridPlacement::from_line_index(index),
        end: GridPlacement::from_span(span),
    }
}
fn size(width: f32, height: f32) -> Size<Dimension> {
    Size {
        width: Dimension::Length(width),
        height: Dimension::Length(height),
    }
}

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let top_left = taffy.new_leaf(Style {
        size: size(100., 50.),
        grid_row: line(1, 2),
        grid_column: line(2, 1),
        ..Default::default()
    })?;
    let top_right = taffy.new_leaf(Style {
        size: size(40., 30.),
        grid_row: line(1, 1),
        grid_column: line(2, 2),
        ..Default::default()
    })?;

    let style = Style {
        display: Display::Grid,
        size: size(360.0, 640.0),
        justify_content: Some(JustifyContent::Start),
        grid_template_columns: vec![auto(), fr(1.), fr(1.)],
        grid_template_rows: vec![auto(), fr(1.), fr(1.)],
        ..Default::default()
    };
    // println!("{:#?}", style.grid_template_columns);

    let node = taffy.new_with_children(style, &[top_left, top_right])?;
    // grid_template_columns: vec![auto(), auto(), auto(), fr(1.0)],
    // grid_template_rows: vec![auto(), auto(), auto(), fr(1.0)],

    taffy.compute_layout(
        node,
        Size {
            height: AvailableSpace::Definite(640.0),
            width: AvailableSpace::Definite(320.0),
        },
    )?;
    print_tree(&taffy, node);
    Ok(())
}
