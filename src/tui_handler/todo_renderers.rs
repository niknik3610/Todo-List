use std::{io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout,
    style::{Color, Modifier, Style},
    widgets, Terminal,
};

use super::tui_handler::AddState;

pub enum BufferType<'a> {
    None,
    AddingTask(&'a str),
    CompletingTask(&'a str),
    UncompletingTask(&'a str),
    Error(&'a str),
}

pub fn render_main(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    buffer: BufferType,
    todo_items: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_contents = match buffer {
        BufferType::None => "Command Mode".to_owned(),
        BufferType::AddingTask(b) => "Adding: ".to_owned() + b,
        BufferType::CompletingTask(b) => "CompletingTask: ".to_owned() + b,
        BufferType::UncompletingTask(b) => "UncompletingTask: ".to_owned() + b,
        BufferType::Error(e) => "Error: ".to_owned() + e,
    };

    terminal
        .draw(|rec| {
            let size = rec.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        layout::Constraint::Length(3), //Adding
                        layout::Constraint::Min(2),    //Content
                        layout::Constraint::Length(3), //Footer
                    ]
                    .as_ref(),
                )
                .split(size);

            let header = widgets::Paragraph::new("TODO LIST")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(widgets::BorderType::Plain),
                );

            let content = widgets::Paragraph::new(todo_items.clone())
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(widgets::BorderType::Plain),
                );

            let command_buffer = widgets::Paragraph::new(command_contents)
                .style(
                    Style::default()
                        .fg(if let BufferType::Error(_) = buffer {
                            Color::Red
                        } else {
                            Color::LightCyan
                        })
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Commands")
                        .border_type(widgets::BorderType::Plain),
                );

            rec.render_widget(header, chunks[0]);
            rec.render_widget(content, chunks[1]);
            rec.render_widget(command_buffer, chunks[2]);
        })
        .expect("Drawing TUI");
    Ok(())
}

pub fn render_adding(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    name_buffer: &str,
    date_buffer: &str,
    todo_items: &String,
    _adding_state: AddState
) -> Result<(), Box<dyn std::error::Error>> {
    terminal
        .draw(|rec| {
            let size = rec.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        layout::Constraint::Length(3), //Adding
                        layout::Constraint::Min(2),    //Content
                        layout::Constraint::Length(3), //Footer
                    ]
                    .as_ref(),
                )
                .split(size);

            let header = widgets::Paragraph::new("TODO LIST")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(widgets::BorderType::Plain),
                );

            let content = layout::Layout::default()
                .direction(layout::Direction::Horizontal)
                .margin(0)
                .constraints([
                    layout::Constraint::Percentage(70),
                    layout::Constraint::Percentage(30),
                ]
                .as_ref(),
                )
                .split(chunks[1]);


            let todos = widgets::Paragraph::new(todo_items.clone())
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(widgets::BorderType::Plain),
                );

            let todo_string = format!(" Task Name: {name_buffer}\n Task Date: {date_buffer}");
            let new_todo = widgets::Paragraph::new(todo_string)
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::LightGreen))
                        .title("New Task")
                        .border_type(widgets::BorderType::Thick),
                );


            let command_buffer = widgets::Paragraph::new("Adding Task")
                .style(
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Commands")
                        .border_type(widgets::BorderType::Plain),
                );

            rec.render_widget(header, chunks[0]);
            rec.render_widget(todos, content[0]);
            rec.render_widget(new_todo, content[1]);
            rec.render_widget(command_buffer, chunks[2]);
        })
        .expect("Drawing TUI");
    Ok(())
}
