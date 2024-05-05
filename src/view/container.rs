use iced::{event, Alignment, Length, Padding};
use iced::widget::{button, container, slider, text, Button, Column, Container, Row, TextInput};
use iced::theme;
use iced::alignment::{Horizontal , Vertical};

use crate::view::style::ButtonStyle;
use crate::{output_text, Field, FieldStr, Message};

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
        true=> output_text(input),
        false=> "Waiting for submission".to_string()
    };
    let column=Column::new()
    .push(text("Averaging Down\n"))
    .push(text(output));

    column
}