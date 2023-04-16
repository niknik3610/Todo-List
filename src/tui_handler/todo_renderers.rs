use std::io::Stdout;

use tui::{
    Terminal, 
    backend::CrosstermBackend, 
    layout, 
    widgets, 
    style::{Style, Color}
};

pub fn render_viewing(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>, 
    todo: &String
    ) -> Result<(), Box<dyn std::error::Error>> {

    terminal.draw(|rec| {
        let size = rec.size();
        let chunks = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .margin(2)
            .constraints([
                         layout::Constraint::Length(3),  //Menu Bar
                         layout::Constraint::Min(2),     //Content
                         layout::Constraint::Length(3)   //Footer
            ]
            .as_ref()
            ) 
            .split(size);                   

        
        let header =
            widgets::Paragraph::new("Nik's TodoList")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let content = 
            widgets::Paragraph::new(todo.clone())
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let footer_copyright_temp = 
            widgets::Paragraph::new("Temp Copyright - Copyright Niklas Harnish")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(widgets::BorderType::Plain)
                );

        rec.render_widget(header, chunks[0]);
        rec.render_widget(content, chunks[1]); 
        rec.render_widget(footer_copyright_temp, chunks[2]);
    }).expect("Drawing TUI");
    Ok(()) 
}

pub fn render_with_buffer(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    output_buffer: &String,
    todo: &String,
    buffer_text: &str
    ) -> Result<(), Box<dyn std::error::Error>> { 

    terminal.draw(|rec| {
        let size = rec.size();
        let chunks = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .margin(2)
            .constraints([
                         layout::Constraint::Length(3),     //Adding
                         layout::Constraint::Min(2),        //Content
                         layout::Constraint::Length(3)      //Footer
            ]
            .as_ref()
            )
            .split(size);                   

        let adding_header =
            widgets::Paragraph::new(buffer_text.to_owned() + &output_buffer[..])
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let content = 
            widgets::Paragraph::new(todo.clone())
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let footer_copyright_temp = 
            widgets::Paragraph::new("Temp Copyright - Copyright Niklas Harnish")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(widgets::BorderType::Plain)
                );

        rec.render_widget(adding_header, chunks[0]);
        rec.render_widget(content, chunks[1]); 
        rec.render_widget(footer_copyright_temp, chunks[2]);
    }).expect("Drawing TUI");
    Ok(()) 
}

pub fn render_error(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    error_buffer: &str,
    todo: &String,
    ) -> Result<(), Box<dyn std::error::Error>> { 

    terminal.draw(|rec| {
        let size = rec.size();
        let chunks = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .margin(2)
            .constraints([
                         layout::Constraint::Length(3),     //Adding
                         layout::Constraint::Min(2),        //Content
                         layout::Constraint::Length(3)      //Footer
            ]
            .as_ref()
            )
            .split(size);                   

        let error_header =
            widgets::Paragraph::new(&error_buffer[..])
            .style(Style::default().fg(Color::Red))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let content = 
            widgets::Paragraph::new(todo.clone())
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(widgets::BorderType::Plain)
                );

        let footer_copyright_temp = 
            widgets::Paragraph::new("Temp Copyright - Copyright Niklas Harnish")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(layout::Alignment::Center)
            .block(
                widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(widgets::BorderType::Plain)
                );

        rec.render_widget(error_header, chunks[0]);
        rec.render_widget(content, chunks[1]); 
        rec.render_widget(footer_copyright_temp, chunks[2]);
    }).expect("Drawing TUI");
    Ok(()) 
}
