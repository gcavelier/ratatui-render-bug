use ratatui::{
    Frame,
    crossterm::{
        self,
        event::{self, KeyCode},
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType::Rounded, Borders, Clear, Padding, Paragraph, Wrap},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut exit = false;
    let mut popup = false;

    ratatui::run(|terminal| {
        while !exit {
            terminal.draw(|frame| render(frame, popup))?;

            // Wait for an event
            let event = crossterm::event::read().unwrap();

            match event {
                crossterm::event::Event::Key(key_event) => {
                    if key_event.kind == event::KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Esc => exit = true,
                            KeyCode::Char('d') => popup = true,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    })
}

#[rustfmt::skip]
pub fn render(frame: &mut Frame, popup: bool) {
    frame.render_widget(
        Paragraph::new("TEST\n123456789")
            .block(Block::default().borders(Borders::ALL).border_type(Rounded))
            .fg(Color::LightBlue) // If this line is commented, render_popup() is ok!
        ,frame.area(),
    );

    if popup {
        render_popup(
            frame,
            "⚠️ Test ⚠️",
            Paragraph::new(format!("message 0123456798 message")),
        );
    }
}

fn render_popup(frame: &mut Frame, title: &str, msg: Paragraph) {
    let area = Rect::new(3, 3, 30, 5);

    let top_block = Block::default()
        .title(title)
        .title_style(Style::new().gray())
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_style(Color::LightBlue)
        .border_type(Rounded);

    frame.render_widget(Clear, area);

    let [top_chunk, bottom_chunk] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]).areas(area);

    frame.render_widget(
        msg.wrap(Wrap { trim: false })
            .block(top_block.padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            })),
        top_chunk,
    );

    // Render bottom border
    let bottom_block = Block::default()
        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
        .border_style(Color::LightBlue)
        .border_type(Rounded);
    frame.render_widget(bottom_block, bottom_chunk);
}
