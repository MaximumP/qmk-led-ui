use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen};

fn render_title(frame: &mut Frame, rect: &Rect) {
    let title = Paragraph::new(Text::styled(
        "QMK Led Control",
        Style::default().fg(Color::Green),
    ))
    .centered()
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, *rect);
}

fn render_color_picker(frame: &mut Frame, rect: &Rect, app: &App) {
    let red: u8 = 90;
    let green: u8 = 10;
    let blue: u8 = 185;
    let average: u16 = (red as u16 + green as u16 + blue as u16) / 3;
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
        .value(average.into())
        .text_value(String::from("Result"))
        .style(Style::new().fg(Color::Rgb(red, green, blue)));

    let color_picker_title = Line::from("Color Picker");
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title(color_picker_title);
    if app.current_screen == CurrentScreen::ColorPicker {
        block = block.border_style(Style::default().fg(Color::Green));
    }
    let color_bar_chart = BarChart::default()
        .max(255)
        .data(BarGroup::default().bars(&[red_bar, green_bar, blue_bar, result_color]))
        .block(block)
        .bar_width(10);

    frame.render_widget(color_bar_chart, *rect);
}

pub fn render_device_picker(frame: &mut Frame, rect: &Rect, app: &App) {
    let mut list_items = Vec::<ListItem>::new();
    for device in app.devices() {
        let manufacturer = match device.manufacturer_string() {
            Some(manufacturer) => manufacturer,
            _ => "Unkown",
        };
        let product = match device.product_string() {
            Some(product) => product,
            _ => "Unknown",
        };
        list_items.push(ListItem::new(Line::from(Span::default().content(format!(
            "{} {} ({} {})",
            manufacturer,
            product,
            device.vendor_id(),
            device.product_id()
        )))));
    }
    let mut block = Block::default().borders(Borders::ALL);
    if app.current_screen == CurrentScreen::SelectDevice {
        block = block.border_style(Style::default().fg(Color::Green));
    }
    let device_list = List::new(list_items).block(block);
    frame.render_widget(device_list, *rect);
}

pub fn ui(frame: &mut Frame, app: &App) {
    let [top, middle, bottom] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .spacing(1)
    .areas(frame.area());

    render_title(frame, &top);

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .spacing(1)
            .areas(middle);

    let [left_top, left_bottom] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(1)
            .areas(left);

    render_device_picker(frame, &right, &app);
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

    render_color_picker(frame, &left_bottom, app);

    let left_block = Block::default().borders(Borders::ALL);
    let device_list = List::new(list_items).block(left_block);
    frame.render_widget(device_list, left_top);

    let hello_world = Line::from("Hello world");
    frame.render_widget(hello_world, right);

    let key_hint = {
        match app.current_screen {
            _ => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let footer = Paragraph::new(Line::from(key_hint))
        .block(Block::default().borders(Borders::ALL))
        .centered();
    frame.render_widget(footer, bottom);
}
