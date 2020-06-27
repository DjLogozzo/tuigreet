mod prompt;

use std::{
    error::Error,
    io::{self, Write},
};

use chrono::prelude::*;
use termion::{cursor::Goto, raw::RawTerminal};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Paragraph, Text},
    Terminal,
};

use crate::Greeter;

const EXIT: &'static str = "Exit";
const COMMAND: &'static str = "SESSION";

pub fn draw(
    terminal: &mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
    greeter: &mut Greeter,
) -> Result<(), Box<dyn Error>> {
    if greeter.working {
        terminal.hide_cursor()?;
    } else {
        terminal.show_cursor()?;
    }

    let mut cursor: Option<(u16, u16)> = None;

    terminal.draw(|mut f| {
        let size = f.size();
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(size);

        if greeter.config().opt_present("time") {
            let time_text = [Text::raw(get_time())];
            let time = Paragraph::new(time_text.iter()).alignment(Alignment::Center);

            f.render_widget(time, chunks[0]);
        }

        let command = greeter.command.clone().unwrap_or("-".to_string());
        let status_text = [
            status_label("ESC"),
            status_value(format!(" {} ", EXIT)),
            status_label(COMMAND),
            status_value(format!(" {} ", command)),
        ];
        let status = Paragraph::new(status_text.iter());

        f.render_widget(status, chunks[2]);

        cursor = self::prompt::draw(greeter, &mut f).ok();
    })?;

    if let Some(cursor) = cursor {
        write!(terminal.backend_mut(), "{}", Goto(cursor.0, cursor.1))?;
    }

    io::stdout().flush()?;

    Ok(())
}

fn get_time() -> String {
    Local::now().format("%b, %d %h %Y - %H:%M").to_string()
}

fn status_label<'s, S>(text: S) -> Text<'s>
where
    S: Into<String>,
{
    Text::styled(text.into(), Style::default().modifier(Modifier::REVERSED))
}

fn status_value<'s, S>(text: S) -> Text<'s>
where
    S: Into<String>,
{
    Text::raw(text.into())
}