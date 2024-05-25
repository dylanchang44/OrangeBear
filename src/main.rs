use iced::{Alignment, Element, Length, Padding, Renderer, Sandbox, Settings};
use iced::widget::{column, container, pick_list, Column, PickList, Row};
use iced::theme::{self, Theme};

use view::container::*;
use view::style::*;
use model::model::*;

mod view;
mod model;
mod util;
fn main() -> iced::Result{
    Bear::run(Settings::default())
}

//first off, create a struct for app state
struct Bear{
    pre_fields: PreField,
    fields: Field,
    theme: Theme,
}

#[derive(Debug, Clone)]
struct PreField{
    position: String,
    start_price: String,
    dip_price: String,
    resolution: u32,
    model:Model,
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
            pre_fields: PreField{
                position: String::new(),
                start_price: String::new(),
                dip_price: String::new(),
                resolution: 4,
                model: Model::FlatHat,
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
                self.pre_fields.position=position;
                self.pre_fields.start_price=start;
                self.pre_fields.dip_price=dip;
            }
            Message::SliderChange(val) =>{
                self.pre_fields.resolution=val;
            }
            Message::ModelSelect(model) =>{
                self.pre_fields.model=model;
            }
            Message::Submit => {
                self.loading_input().unwrap();
            }
            Message::ToggleTheme => {
                self.theme= if self.theme == Theme::Light {Theme::Dark}
                else{ Theme::Light }
            }
            
        }
    }

    fn view(&self) -> Element<Message> {
        let text_insert= insert_block(&self.pre_fields);
        let slider=reso_slider(self.pre_fields.resolution);
        let pick_list:PickList<'static, Model, [Model; 2], Model, Message, Theme, Renderer>=pick_list(Model::ALL, Some(self.pre_fields.model), Message::ModelSelect).placeholder("Choose a model");
        let submit=submit_btn("RUN", Message::Submit);
        let input_column=column![text_insert, slider, pick_list, submit].padding(Padding::from([30,20]))
        .align_items(Alignment::Center)
        .spacing(20);

        let output=res_block(&self.fields).padding(Padding::from([20,15]));
        //max width 500
        let input_box=container(input_column).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)));
        let output_box=container(output).padding(Padding::from(20)).style(theme::Container::Custom(Box::new(ContainerStyle)));
        let box_row=Row::new().push(input_box).push(output_box).spacing(50);
        
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
    fn loading_input(&mut self) -> Result<(), Error>{
        let inputs=self.pre_fields.clone();
        //turn str into u32
        let position: u32 = inputs.position.parse().map_err(|_| Error::NotIntegerError)?;
        let start: u32 = inputs.start_price.parse().map_err(|_| Error::NotIntegerError)?;
        let dip: u32 = inputs.dip_price.parse().map_err(|_| Error::NotIntegerError)?;

        if position <= 0 && start<=dip {
            return Err(Error::ValueError);
        }

        // Update self.fields if all checks pass
        self.fields.position = position;
        self.fields.start_price = start;
        self.fields.dip_price = dip;
        self.fields.resolution = inputs.resolution;
        self.fields.model = inputs.model;
        self.fields.integrity = true;
        Ok(())
    }
}
