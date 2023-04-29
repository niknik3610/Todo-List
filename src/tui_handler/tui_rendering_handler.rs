use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{self, Rect},
    style::{Color, Modifier, Style},
    widgets::{self, Paragraph}, Terminal,
};
use super::tui_handler::DateState;

const TODO_SIZE: u16 = 30;

pub enum BufferType<'a> {
    None,
    AddingTask(&'a str),
    CompletingTask(&'a str),
    UncompletingTask(&'a str),
    Error(&'a str),
}

#[derive(Clone)]
pub struct TodoItems {
    pub indexes: String,
    pub todo_content: String,
    pub check_boxes: String,
}
impl TodoItems {
    pub fn new(indexes: String, todo_content: String, check_boxes: String) -> TodoItems {
        TodoItems {indexes, todo_content, check_boxes} 
    }
}

pub fn render_main(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    buffer: BufferType,
    todo_items: &TodoItems,
) -> io::Result<()> {
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

            let (content, todo_content, indexes, todos, completions) =   
                generate_content(&chunks, todo_items).unwrap();
 
            let header = widgets::Paragraph::new("TODO LIST")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(widgets::BorderType::Plain),
                    );
   
            let empty_left = widgets::Paragraph::new("")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
                    .style(Style::default().fg(Color::White))
                    .border_type(widgets::BorderType::Plain),
                    );
 
            let empty_right = widgets::Paragraph::new("")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
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
            rec.render_widget(empty_left, content[0]);
           
            rec.render_widget(indexes, todo_content[0]);
            rec.render_widget(todos, todo_content[1]);
            rec.render_widget(completions, todo_content[2]);

            rec.render_widget(empty_right, content[2]);
            rec.render_widget(command_buffer, chunks[2]);
        })
        .expect("Drawing TUI");
    Ok(())
}

pub fn render_adding(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    name_buffer: &str,
    todo_items: &TodoItems,
) -> io::Result<()> {
    let todo_string = format!(" Task Name: {name_buffer}\n ");
    
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

            let empty_left = widgets::Paragraph::new("")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
                    .style(Style::default().fg(Color::White))
                    .border_type(widgets::BorderType::Plain),
                    );

            let (content, todo_content, indexes, todos, completions) =   
                generate_content(&chunks, todo_items).unwrap();

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
            rec.render_widget(empty_left, content[0]);
            
            rec.render_widget(indexes, todo_content[0]);
            rec.render_widget(todos, todo_content[1]);
            rec.render_widget(completions, todo_content[2]);

            rec.render_widget(new_todo, content[2]);
            rec.render_widget(command_buffer, chunks[2]);
        })
        .expect("Drawing TUI");
    Ok(())
}

pub fn render_adding_date(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    name_buffer: &str,
    date_buffer: &str,
    date_storage_buff: &str,
    todo_items: &TodoItems,
    date_state: &DateState,
) -> io::Result<()> {
    let mut todo_string = format!(" Task Name: {name_buffer}\n ");
    match date_state {
        DateState::Year => todo_string += &*("Enter Year: ".to_owned() + date_buffer),
        DateState::Month => todo_string += &*("Enter Month: ".to_owned() + date_buffer),
        DateState::Day => todo_string += &*("Enter Day: ".to_owned() + date_buffer),
        DateState::Time => todo_string += &*("Enter Time: ".to_owned() + date_buffer),
    }
    todo_string += &*format!("\n {date_storage_buff}");

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


            let (content, todo_content, indexes, todos, completions) =   
                generate_content(&chunks, todo_items).unwrap();
            
            let empty_left = widgets::Paragraph::new("")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
                    .style(Style::default().fg(Color::White))
                    .border_type(widgets::BorderType::Plain),
                    );

            let new_todo = widgets::Paragraph::new(todo_string)
                .style(Style::default().fg(Color::LightCyan))
                .alignment(layout::Alignment::Left)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(Style::default().fg(Color::LightGreen))
                        .title("New Task With Date")
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
            
            //header area
            rec.render_widget(header, chunks[0]);

            //center area
            rec.render_widget(empty_left, content[0]);
            rec.render_widget(new_todo, content[2]);
            //todo content (in the middle)
            rec.render_widget(indexes, todo_content[0]);
            rec.render_widget(todos, todo_content[1]);
            rec.render_widget(completions, todo_content[2]);
            
            //bottom area
            rec.render_widget(command_buffer, chunks[2]);
        })
        .expect("Drawing TUI");
    Ok(())
}

fn generate_content<'a>(
    chunks: &Vec<Rect>,
    todo_items: &'a TodoItems) -> 
io::Result<(Vec<Rect>, Vec<Rect>, Paragraph<'a>, Paragraph<'a>, Paragraph<'a>)> {
    let content = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .margin(0)
        .constraints(
            [
            layout::Constraint::Percentage((100 - TODO_SIZE)/2),
            layout::Constraint::Percentage(TODO_SIZE),
            layout::Constraint::Percentage((100 - TODO_SIZE)/2),
            ]
            .as_ref(),
            )
        .split(chunks[1]);

    const MARGIN_SIZE: u16 = 10;
    let todo_content = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .margin(1)
        .constraints(
            [
            layout::Constraint::Percentage(MARGIN_SIZE),
            layout::Constraint::Percentage(100 - (MARGIN_SIZE * 2)),
            layout::Constraint::Percentage(MARGIN_SIZE),
            ]
            .as_ref(),
            )
        .split(content[1]);
    
    let indexes = widgets::Paragraph::new(&*todo_items.indexes)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(layout::Alignment::Left)
        .block(
            widgets::Block::default()
            .borders(widgets::Borders::LEFT)
            .style(Style::default().fg(Color::White))
            .border_type(widgets::BorderType::Thick),
            );
 
    let todos = widgets::Paragraph::new(&*todo_items.todo_content)
    .style(Style::default().fg(Color::LightCyan))
    .alignment(layout::Alignment::Left)
    .block(
        widgets::Block::default()
        .borders(widgets::Borders::NONE)
        .style(Style::default().fg(Color::White))
        .border_type(widgets::BorderType::Plain),
        );

    let completions = widgets::Paragraph::new(&*todo_items.check_boxes)
    .style(Style::default().fg(Color::LightCyan))
    .alignment(layout::Alignment::Right)
    .block(
        widgets::Block::default()
        .borders(widgets::Borders::RIGHT)
        .style(Style::default().fg(Color::White))
        .border_type(widgets::BorderType::Thick),
        ); 
    return Ok((content, todo_content, indexes, todos, completions));
}
