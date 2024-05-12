use iced::{alignment::{Horizontal , Vertical}, theme, Alignment, Length, Padding};
use iced::widget::{button, container, slider, text, Text, Button, Column, Container, Row, TextInput};

use crate::{output_visual, view::style::{ButtonStyle, ColorRgb}};
use crate::{Field, FieldStr, Message};

// input field
pub fn input_field(_placeholder: &str, _vale: &str) -> TextInput<'static, Message>{
    TextInput::new(_placeholder, _vale)
    .width(Length::Fixed(500.0))
    .padding(Padding::from(10))
    .line_height(text::LineHeight::Relative(1.75))
}

pub fn submit_btn(name: &str, event: Message) -> Button<Message>{
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    ).on_press(event)
    .width(Length::Fixed(250.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

pub fn page_footer() -> Container<'static, Message>{
    let footer = Row::new().push(
    button("Toggle Theme").on_press(Message::ToggleTheme).style(
            theme::Button::Custom(Box::new(ButtonStyle::ThemeButton))))
            .align_items(Alignment::Center);
    container(footer).center_x().center_y()
}

pub fn reso_slider(slider_val: u32) -> Column<'static, Message>{
    let slider=Column::new()
    .push(text("Averaging down resolition:"))
    .push(slider(4..=10, slider_val, Message::SliderChange)).width(Length::Fixed(500.0))
    .push(text(slider_val.to_string()).width(Length::Fill).horizontal_alignment(Horizontal::Center));
    slider
}

pub fn insert_block(fieldstr: &FieldStr) -> Column<Message>{
    let column=Column::new()
    .push(text("Orange Bear"))
    .push(
        input_field("Overall Position", &fieldstr.position)
        .on_input(|position|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldStrChange(position, fieldstr.start_price.clone(), fieldstr.dip_price.clone())
        })
    )
    .push(
        input_field("Preferred Starting price", &fieldstr.start_price)
        .on_input(|start|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldStrChange(fieldstr.position.clone(), start, fieldstr.dip_price.clone())
        })
    )
    .push(
        input_field("Estimated Dip Price", &fieldstr.dip_price)
        .on_input(|dip|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldStrChange(fieldstr.position.clone(), fieldstr.start_price.clone(), dip)
        })
    ).padding(Padding::from([50,20]))
    .align_items(Alignment::Center)
    .spacing(40);

    column
}

pub fn res_block(input: &Field) -> Column<Message>{
    //check integrity to decide whether output text
    let output=match input.integrity{
        true=> output_visual(input),
        false=> Column::new().push(text("Waiting for submission\n"))
    };
    let column=Column::new()
    .push(text("Averaging Down\n")).align_items(Alignment::Center)
    .push(output).spacing(50);

    column
}

pub fn order_visual(order_vec: Vec<(u32,u32)>) -> Column<'static, Message> {
    let mut visual_col = Column::new();
    let orange_rgb: Vec<ColorRgb>=vec![
        (255.0, 175.0, 75.0),
        (255.0, 162.0, 60.0),
        (255.0, 149.0, 45.0),
        (255.0, 136.0, 30.0),
        (255.0, 123.0, 15.0),
        (237.0, 111.0, 0.0),
        (219.0, 102.0, 0.0),
        (200.0, 90.0, 0.0),
        (147.0, 78.0, 0.0),
        (99.0, 58.0, 0.0),
    ];
    let color_vec=match order_vec.len(){
        1..=8 => {orange_rgb[2..(order_vec.len()+2)].to_vec()},
        _ => orange_rgb[..order_vec.len()].to_vec()};
    let mut price_button_text:String;
    for (idx, order) in order_vec.iter().enumerate() {
        price_button_text=format!("Order {} shares", order.1);
        let rgb_covert=(color_vec[idx].0/255.0, color_vec[idx].1/255.0, color_vec[idx].2/255.0);
        let price_button=Button::new(
            text(price_button_text)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(16)
        )
        .width(Length::Fixed(500.0 - (9.0 - (idx as f32)) * 35.0))
        .height(Length::Fixed(35.0))
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::PriceButton(rgb_covert))));

        let order_bar = Row::new()
            .push(Text::new(order.0.to_string()))
            .push(price_button).spacing(10).align_items(Alignment::Center);

            visual_col = visual_col.push(order_bar);
    }

    visual_col
    .align_items(Alignment::Start)
    .spacing(30)
}