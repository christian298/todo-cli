use super::todo::Todo;
use super::Filter;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

pub fn draw_todo_list(todos: Vec<Todo>, filter: Filter) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("");

    terminal.clear()?;

    terminal.draw(|mut f| {
        let size = f.size();

        let normal_style = Style::default().fg(Color::White);
        let highlighted_style = Style::default().fg(Color::Gray);

        let header = ["ID", "Title", "Done"];

        let rows = todos
            .iter()
            .enumerate()
            .filter(|&item| {
                filter == Filter::All
                    || (item.1.done && filter == Filter::Done)
                    || (!item.1.done && filter == Filter::Open)
            })
            .map(|(i, item)| {
                if i % 2 == 0 {
                    Row::StyledData(
                        vec![
                            item.id.to_string(),
                            item.title.clone(),
                            item.done.to_string(),
                        ]
                        .into_iter(),
                        normal_style,
                    )
                } else {
                    Row::StyledData(
                        vec![
                            item.id.to_string(),
                            item.title.clone(),
                            item.done.to_string(),
                        ]
                        .into_iter(),
                        highlighted_style,
                    )
                }
            });
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(size);
        Table::new(header.into_iter(), rows)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .header_style(Style::default().fg(Color::Blue))
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(60),
                Constraint::Percentage(10),
            ])
            .render(&mut f, rects[0]);
    })?;

    Ok(())
}
