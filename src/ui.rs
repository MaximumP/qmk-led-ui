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
    let average: u16 =
        (app.rgb_color.red as u16 + app.rgb_color.green as u16 + app.rgb_color.blue as u16) / 3;
    let red_bar = Bar::default()
        .value(app.rgb_color.red.into())
        .text_value(String::from("Red"))
        .style(Style::new().fg(Color::Red));
    let green_bar = Bar::default()
        .value(app.rgb_color.green.into())
        .text_value(String::from("Green"))
        .style(Style::new().fg(Color::Green));
    let blue_bar = Bar::default()
        .value(app.rgb_color.blue.into())
        .text_value(String::from("Blue"))
        .style(Style::new().fg(Color::Blue));
    let result_color = Bar::default()
        .value(average.into())
        .text_value(String::from("Result"))
        .style(Style::new().fg(Color::Rgb(
            app.rgb_color.red,
            app.rgb_color.green,
            app.rgb_color.blue,
        )));

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

fn render_device_picker(frame: &mut Frame, rect: &Rect, app: &App) {
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

fn render_key_hint(frame: &mut Frame, rect: &Rect, app: &App) {
    let quit = Span::styled("(q) to quit", Style::default().fg(Color::Red));
    let select = Span::styled("(s) select device", Style::default().fg(Color::Green));
    let pick = Span::styled("(p) pick color", Style::default().fg(Color::Green));
    let main = Span::styled("(ESC) main", Style::default().fg(Color::Green));
    let line: Line = match app.current_screen {
        CurrentScreen::Main => {
            let spans = vec![quit, select, pick];
            Line::from(spans)
        }
        CurrentScreen::ColorPicker => {
            let spans = vec![
                quit,
                select,
                main,
                Span::styled("(r) Red", Style::default().fg(Color::Red)),
                Span::styled("(g) Green", Style::default().fg(Color::Green)),
                Span::styled("(b) Blue", Style::default().fg(Color::Blue)),
            ];
            Line::from(spans)
        }
        CurrentScreen::SelectDevice => {
            let spans = vec![quit, pick, main];
            Line::from(spans)
        }
        CurrentScreen::LedModesList => Line::from(Span::default().content("todo")),
        CurrentScreen::DeviceInfo => Line::from(Span::default().content("todo")),
    };

    let paragraph = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL))
        .centered();
    frame.render_widget(paragraph, *rect);
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

    render_key_hint(frame, &bottom, app);
}
