use std::default;
use iced::{event, Renderer, Alignment, Background, Border, Element, Length, Padding, Sandbox, Settings, Shadow, Vector};
use iced::widget::{button, column, container, pick_list, slider, text, text_input, Button, Column, Container, PickList, Row, Text, TextInput};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Field{
    position: u8,
    start_price: u8,
    dip_price: u8,
    resolution: u8,
    model:Model,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Model{
    #[default]
    FlatHat,
    Pyramid
}

impl Model{
    const ALL: [Model;2]=[
        Model::FlatHat,
        Model::Pyramid,
    ];
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Model::FlatHat => "Flat hat",
                Model::Pyramid => "Pyramid"})
            }
        }

#[derive(Debug, Clone)]
enum Message {
    FieldStrChange(String, String, String), 
    SliderChange(u8),
    ModelSelect(Model),
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
                resolution: 4,
                model: Model::FlatHat,
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
            Message::FieldStrChange(position,start,dip) => {
                self.fields_str.position=position;
                self.fields_str.start_price=start;
                self.fields_str.dip_price=dip;
            }
            Message::SliderChange(val) =>{
                self.fields.resolution=val;
            }
            Message::ModelSelect(model) =>{
                self.fields.model=model;
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
        let text_insert= insert_block(&self.fields_str);
        let slider=reso_slider(self.fields.resolution);
        let pick_list:PickList<'static, Model, [Model; 2], Model, Message, Theme, Renderer>=pick_list(Model::ALL, Some(self.fields.model), Message::ModelSelect).placeholder("Choose a model");
        let submit=submit_btn("RUN", Message::Submit);

        let column=column![text_insert,slider, pick_list, submit].padding(Padding::from([50,20]))
        .align_items(Alignment::Center)
        .spacing(40);

        let input_box=container(column).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)));
        //let btn = submit_btn("Toggle Theme", Message::ToggleTheme);
        let wrapper = Column::new()
        .spacing(50).width(Length::Fill).align_items(Alignment::Start).push(input_box)
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
    .width(Length::Fixed(250.0))
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

fn reso_slider(slider_val: u8) -> Column<'static, Message>{
    let slider=Column::new()
    .push(text("Averaging down resolition:"))
    .push(slider(4..=10, slider_val, Message::SliderChange)).width(Length::Fixed(500.0))
    .push(text(slider_val.to_string()).width(Length::Fill).horizontal_alignment(Horizontal::Center));
    slider
}

fn insert_block(fieldstr: &FieldStr) -> Column<Message>{
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

fn res_block(){

}