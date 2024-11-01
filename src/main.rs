mod r#file;
use file::{ read_words_from_file,  trim_whitespace};

mod r#mcsrvstat;
use mcsrvstat::write_info;

use std::io;
use crossterm::{
    event::{ self, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Style },
    text::{ Span, Spans },
    widgets::{ Block, Borders, BorderType, Cell, Row, Table },
    Terminal,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Reading the lines from the file
    let info = read_words_from_file("serverinfo");

    // Making a vector of color for boolean 
    let mut v_colors : Vec<Color> = Vec::new();
    for i in 7..10 { 
        if info[i] == "true" { v_colors.push(Color::Green); }
        else { v_colors.push(Color::Red); }
    }

    loop {
        terminal.draw( |f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(5)
                .horizontal_margin(10)
                .constraints(
                    [
                        Constraint::Percentage(25), 
                        Constraint::Percentage(5), 
                        Constraint::Percentage(25), 
                        Constraint::Percentage(25),
                        Constraint::Percentage(20)
                    ].as_ref())
                .split(f.size());

            // The first table for the Motto of the day
            let table1 = Table::new(vec![Row::new(vec![Cell::from("")])])
                .block(Block::default().borders(Borders::NONE)).widths(&[Constraint::Percentage(100)]);

            // The first table for the Motto of the day
            let table2 = Table::new(vec![Row::new(vec![Cell::from(trim_whitespace(info[6].clone().as_str()))])])
                .block(Block::default().borders(Borders::NONE)).widths(&[Constraint::Percentage(100)]);

            // First tabel
            let table3 = Table::new(vec![
                Row::new(vec![Cell::from("")]),
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::raw("  Press"),
                        Span::styled(" Q", Style::default().fg(Color::Red)),
                        Span::raw(" to"),
                        Span::styled(" QUIT", Style::default().fg(Color::Red)),
                        Span::raw("."),
                    ]))
                ]),
                
                Row::new(vec![Cell::from("  ────────────────")]),
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::raw("  Online : "),
                        Span::styled(info[0].clone(), Style::default().fg(Color::Green)),
                    ]))
                ]),
                Row::new(vec![Cell::from("  IP : ".to_owned() + &info[1])]),
                Row::new(vec![Cell::from("  Port : ".to_owned() + &info[2])]),
                Row::new(vec![Cell::from("  Version : ".to_owned() + &info[3])]),
                Row::new(vec![Cell::from("  API version : ".to_owned() + &info[12])]),
            ])
            .block(
                Block::default()
                    .title(" ".to_owned() + &info[4].clone().to_uppercase() + " ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded))
                    
            .widths(&[Constraint::Percentage(100)]);

            // Second tabel
            let table4 = Table::new(vec![
                Row::new(vec![Cell::from("")]),
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::raw("  Ping : "),
                        Span::styled(info[7].clone(), Style::default().fg(v_colors[0])),
                    ]))
                ]),
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::raw("  Query : "),
                        Span::styled(info[8].clone(), Style::default().fg(v_colors[1])),
                    ]))
                ]),
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::raw("  Srv : "),
                        Span::styled(info[9].clone(), Style::default().fg(v_colors[2])),
                    ]))
                ]),

                Row::new(vec![Cell::from("")]),
                Row::new(vec![Cell::from("  Players : ".to_owned() + &info[5])]),
                Row::new(vec![Cell::from("  Cachetime : ".to_owned() + &info[10])]),
                Row::new(vec![Cell::from("  Cacheexpire : ".to_owned() + &info[11])]),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White)))
            .widths(&[Constraint::Percentage(100)]);

            let table5 = Table::new(vec![Row::new(vec![Cell::from("")])])
                .block(Block::default().borders(Borders::NONE)).widths(&[Constraint::Percentage(100)]);

            // Showing the tables
            f.render_widget(table1, chunks[0]);
            f.render_widget(table2, chunks[1]);
            f.render_widget(table3, chunks[2]);
            f.render_widget(table4, chunks[3]);
            f.render_widget(table5, chunks[3]);
        })?;

        // Exit the app by pressing 'q'
        if let event::Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Terminal config
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend: CrosstermBackend<_> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<_> = Terminal::new(backend)?;

    // Launch app
    let _ = write_info();
    let res = run_app(&mut terminal);

    // Restore the terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}