#![allow(unused_imports)]
use taffy::{
    print_tree,
    style_helpers::{auto, fr, TaffyAuto, TaffyGridLine, TaffyGridSpan},
    AlignItems, AvailableSpace, Dimension, Display, GridPlacement, JustifyContent, Line, MinMax,
    MinTrackSizingFunction, NodeId, Size, Style, TaffyError, TaffyTree, TrackSizingFunction,
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
    let node = compute_grid(&mut taffy, false)?;

    println!("Grid with two components");
    print_tree(&taffy, node);

    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let node = compute_grid(&mut taffy, true)?;

    println!("Grid with bottom component");
    print_tree(&taffy, node);

    Ok(())
}

fn compute_grid(
    taffy: &mut TaffyTree<()>,
    with_bottom_component: bool,
) -> Result<NodeId, TaffyError> {
    let top_left = taffy.new_leaf(Style {
        size: size(100., 50.),
        grid_row: line(1, 2),
        grid_column: line(1, 1),
        ..Default::default()
    })?;
    let top_right = taffy.new_leaf(Style {
        size: size(40., 30.),
        grid_row: line(1, 1),
        grid_column: line(2, 2),
        ..Default::default()
    })?;

    let mut children = vec![top_left, top_right];
    if with_bottom_component {
        let bottom_span_two = taffy.new_leaf(Style {
            size: size(120., 20.),
            grid_row: line(3, 1),
            grid_column: line(1, 2),
            ..Default::default()
        })?;
        children.push(bottom_span_two);
    }

    let grid_style = Style {
        display: Display::Grid,
        size: size(320.0, 640.0),
        justify_content: Some(JustifyContent::Start),
        justify_items: Some(AlignItems::Start),
        grid_template_columns: vec![auto(), auto(), fr(1.)],
        grid_template_rows: vec![auto(), auto(), auto(), fr(1.)],
        ..Default::default()
    };

    let node = taffy.new_with_children(grid_style, &children)?;
    taffy.compute_layout(
        node,
        Size {
            height: AvailableSpace::Definite(640.0),
            width: AvailableSpace::Definite(320.0),
        },
    )?;
    Ok(node)
}
