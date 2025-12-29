use ratatui::buffer::Buffer;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;

fn main() {
    let prev_no_color = Buffer::with_lines([Line::from("   ")]);
    let prev_with_color = Buffer::with_lines([Line::from("   ").fg(Color::Red)]);
    let next = Buffer::with_lines([Line::from("⚠️")]);

    let diff = prev_no_color.diff(&next);

    println!("diff with prev_no_color:");
    for (x, y, cell) in &diff {
        println!("({}, {}): {:?}", x, y, cell);
    }

    assert!(
        !diff.iter().any(|(x, y, _)| *x == 1 && *y == 0),
        "Diff should not include trailing cell (1,0) when only style changed. \
            Found updates: {:?}",
        diff.iter()
            .map(|(x, y, c)| (x, y, c.symbol(), c.fg))
            .collect::<Vec<_>>()
    );

    let diff = prev_with_color.diff(&next);

    println!("\n\ndiff with prev_with_color:");
    for (x, y, cell) in &diff {
        println!("({}, {}): {:?}", x, y, cell);
    }

    assert!(
        !diff.iter().any(|(x, y, _)| *x == 1 && *y == 0),
        "Diff should not include trailing cell (1,0) when only style changed. \
            Found updates: {:?}",
        diff.iter()
            .map(|(x, y, c)| (x, y, c.symbol(), c.fg))
            .collect::<Vec<_>>()
    );
}
