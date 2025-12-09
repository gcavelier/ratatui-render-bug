use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Widget;

fn main() {
    let area = Rect::new(0, 0, 20, 1);

    // Previous buffer with content (simulating a list view)
    let mut prev_buf = Buffer::empty(area);
    prev_buf.set_string(0, 0, "NAME                ", Style::default());

    // New buffer with VS16 emoji (simulating a popup)
    let mut next_buf = Buffer::empty(area);
    Line::from(" ⚠️ Title ").render(area, &mut next_buf); // ⚠️ = U+26A0 + U+FE0F

    let diff = prev_buf.diff(&next_buf);

    // BUG: diff contains update for position 2 with " " which overwrites
    // the right half of the wide emoji at position 1
    for (x, y, cell) in &diff {
        println!("({}, {}): {:?}", x, y, cell.symbol());
    }
}
