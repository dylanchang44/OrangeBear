use iced::{event, Alignment, Background, Border, Element, Length, Padding, Sandbox, Settings, Shadow, Vector};
use iced::widget::{button, container, text, text_input, Button, Column, Container, Row, Text, TextInput};

use iced::theme::{self, Theme};
use iced::alignment::{Horizontal , Vertical};

fn main() -> iced::Result{
    Bear::run(Settings::default())
}

//first off, create a struct for app state
struct Bear{
    fields_str: FieldStr,
    fields: Field,
    theme: Theme,
}

#[derive(Debug, Clone)]
struct FieldStr{
    position: String,
    start_price: String,
    dip_price: String,
}

#[derive(Debug, Clone)]
struct Field{
    position: u8,
    start_price: u8,
    dip_price: u8,
    model: OrderModel,
}

#[derive(Debug, Clone)]
enum OrderModel{
    Flat,
    Pyramid,
    SteepTemple,
}

#[derive(Debug, Clone)]
enum Message {
    FieldChange(String, String, String), 
    Submit, // trigger to calculate bid distribution
    Clear, // reset to default
    ToggleTheme, // change between light/dark
}

impl Sandbox for Bear{
    type Message = Message;

    fn new() -> Self { // set default setting
        Self {
            fields: Field{
                position: 0,
                start_price: 0,
                dip_price: 0,
                model: OrderModel::Flat,
            },
            fields_str: FieldStr{
                position: String::new(),
                start_price: String::new(),
                dip_price: String::new(),
            },
            theme: Theme::Light,
        }
    }

    fn title(&self) -> String {
        String::from("Orange Bear")
    }

    fn theme(&self) -> Theme{
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message{
            Message::FieldChange(position,start,dip) => {
                self.fields_str.position=position;
                self.fields_str.start_price=start;
                self.fields_str.dip_price=dip;
            }
            Message::Submit => {}
            Message::Clear => {}
            Message::ToggleTheme => {
                self.theme= if self.theme == Theme::Light {Theme::Dark}
                else{ Theme::Light }
            }
            
        }
    }

    fn view(&self) -> Element<Message> {
        let content= insert_block(&self.fields_str);
        //let btn = submit_btn("Toggle Theme", Message::ToggleTheme);
        let wrapper = Column::new()
        .spacing(50).width(Length::Fill).align_items(Alignment::Center).push(content)
        .push(page_footer());

        container(wrapper)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from(20))
        .center_x().center_y().style(theme::Container::Custom(Box::new(ContainerStyle))).into()
        }

    
}

// setup page components
//define button styling
enum ButtonStyle{
    Standard,
    ThemeButton,
}

impl button::StyleSheet for ButtonStyle{
    type Style= Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
        background: Some(Background::Color(match self {
            Self::Standard=> iced::Color::from_rgb(0.059, 0.463, 0.702),
            Self::ThemeButton=> iced::Color::default()
        })), 
        text_color: {
            if theme == &Theme::Light {
                match self {
                    Self::Standard => iced::Color::WHITE,
                    Self::ThemeButton => iced::Color::BLACK,
                }
            } else {
                match self {
                    Self::Standard => iced::Color::WHITE,
                    Self::ThemeButton => iced::Color::WHITE,
                }
            }
        }, 
        border: match self {
            Self::Standard => Border::with_radius(5),
            Self::ThemeButton => Border::default(),
        }, 
        shadow: match self{
            Self::Standard => Shadow{
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 0.4),
                blur_radius: 20.0,
            },
            Self::ThemeButton => Shadow::default(),
        },
        ..Default::default()
    }
    }
    
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
    
        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }
    
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }
    
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
    
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(iced::Color {
                    a: color.a * 0.5,
                    ..color
                }),
                iced::Background::Gradient(gradient) => {
                    iced::Background::Gradient(gradient.mul_alpha(0.5))
                }
            }),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        container::Appearance { text_color: Default::default(), background: None, border: Border::with_radius(5), 
            shadow: Shadow{
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            } }
    }
}

// input field
fn input_field(_placeholder: &str, _vale: &str) -> TextInput<'static, Message>{
    TextInput::new(_placeholder, _vale)
    .width(Length::Fixed(500.0))
    .padding(Padding::from(10))
    .line_height(text::LineHeight::Relative(1.75))
}

fn submit_btn(name: &str, event: Message) -> Button<Message>{
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    ).on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

fn page_footer() -> Container<'static, Message>{
    let footer = Row::new().push(
    button("Toggle Theme").on_press(Message::ToggleTheme).style(
            theme::Button::Custom(Box::new(ButtonStyle::ThemeButton))))
            .align_items(Alignment::Center);
    container(footer).center_x().center_y()
}

fn insert_block(field: &FieldStr) -> Container<Message>{
    let column=Column::new()
    .push(text("Orange Bear"))
    .push(
        input_field("Overall Position", &field.position)
        .on_input(|position|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldChange(position, field.start_price.clone(), field.dip_price.clone())
        })
    )
    .push(
        input_field("Preferred Starting price", &field.start_price)
        .on_input(|start|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldChange(field.position.clone(), start, field.dip_price.clone())
        })
    )
    .push(
        input_field("Estimated Dip Price", &field.dip_price)
        .on_input(|dip|{
            //when we call the FileChange func, we need to update position text input field only - therefore others remain unchanged
            Message::FieldChange(field.position.clone(), field.start_price.clone(), dip)
        })
    ).push(submit_btn("RUN", Message::Submit))
    .padding(Padding::from([50,20]))
    .align_items(Alignment::Center)
    .spacing(40);

    container(column).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)))
}