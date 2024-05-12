use iced::{Alignment, Element, Length, Padding, Renderer, Sandbox, Settings};
use iced::widget::{column, container, pick_list, Column, PickList, Row};
use iced::theme::{self, Theme};

use view::container::*;
use view::style::*;
use model::model::*;
use util::util::*;

mod view;
mod model;
mod util;
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
    position: u32,
    start_price: u32,
    dip_price: u32,
    resolution: u32,
    model:Model,
    integrity: bool,
}

#[derive(Debug, Clone)]
enum Message {
    FieldStrChange(String, String, String), 
    SliderChange(u32),
    ModelSelect(Model),
    Submit, // trigger to calculate bid distribution
    ToggleTheme, // change between light/dark
}

#[derive(Debug, Clone)]
pub enum Error{
    NotIntegerError,
    ValueError,
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
                integrity: false,
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
            Message::Submit => {
                self.loading_inputstr().unwrap();
            }
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
        let input_column=column![text_insert, slider, pick_list, submit].padding(Padding::from([50,20]))
        .align_items(Alignment::Center)
        .spacing(25);

        let output=res_block(&self.fields).padding(Padding::from([50,20]));
        //max width 500
        let input_box=container(input_column).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)));
        let output_box=container(output).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)));
        let box_row=Row::new().spacing(20).push(input_box).push(output_box);
        
        //row![input_box, output_box];
        let wrapper = Column::new()
        .spacing(50).width(Length::Fill).align_items(Alignment::Center)
        .push(box_row)
        .push(page_footer());

        container(wrapper)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from(20))
        .center_x().center_y().style(theme::Container::Custom(Box::new(ContainerStyle))).into()
        }

    
}

impl Bear{
    fn loading_inputstr(&mut self) -> Result<(), Error>{
        let input_str=self.fields_str.clone();
        //turn str into u32
        let position: u32 = input_str.position.parse().map_err(|_| Error::NotIntegerError)?;
        let start: u32 = input_str.start_price.parse().map_err(|_| Error::NotIntegerError)?;
        let dip: u32 = input_str.dip_price.parse().map_err(|_| Error::NotIntegerError)?;

        if position <= 0 && start<=dip {
            return Err(Error::ValueError);
        }

        // Update self.fields if all checks pass
        self.fields.position = position;
        self.fields.start_price = start;
        self.fields.dip_price = dip;
        self.fields.integrity = true;
        Ok(())
    }
}
