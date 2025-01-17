use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "QMK Led Control",
        Style::default().fg(Color::Green),
    ))
    .centered()
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let mut list_items = Vec::<ListItem>::new();
    for device in app.devices() {
        let manufacturer = match device.manufacturer_string() {
            Some(manufacturer) => manufacturer,
            _ => "Unknown",
        };
        let product = match device.product_string() {
            Some(product) => product,
            _ => "Unknown",
        };
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "{} {} ({} {})",
                manufacturer,
                product,
                device.vendor_id(),
                device.product_id()
            ),
            Style::default().fg(Color::Gray),
        ))));
    }
    let device_list = List::new(list_items);
    frame.render_widget(device_list, left_chunks[0]);

    let red: u8 = 90;
    let green: u8 = 10;
    let blue: u8 = 185;

    let red_bar = Bar::default()
        .value(red.into())
        .text_value(String::from("Red"))
        .style(Style::new().fg(Color::Red));
    let green_bar = Bar::default()
        .value(green.into())
        .text_value(String::from("Green"))
        .style(Style::new().fg(Color::Green));
    let blue_bar = Bar::default()
        .value(blue.into())
        .text_value(String::from("Blue"))
        .style(Style::new().fg(Color::Blue));
    let result_color = Bar::default()
        .value(((red + green + blue) / 3).into())
        .text_value(String::from("Result"))
        .style(Style::new().fg(Color::Rgb(red, green, blue)));

    let color_picker_title = Line::from("Color Picker").centered();
    let color_bar_chart = BarChart::default()
        .max(255)
        .data(BarGroup::default().bars(&[red_bar, green_bar, blue_bar, result_color]))
        .block(Block::new().title(color_picker_title))
        .bar_width(10);

    frame.render_widget(color_bar_chart, left_chunks[1]);

    let key_hint = {
        match app.current_screen {
            _ => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let footer = Paragraph::new(Line::from(key_hint))
        .block(Block::default().borders(Borders::ALL))
        .centered();
    frame.render_widget(footer, chunks[2]);
}
