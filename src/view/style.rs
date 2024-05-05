use iced::{Background, Shadow, Border, Vector};
use iced::widget::{button, container};
use iced::theme::Theme;

//define button styling
pub enum ButtonStyle{
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

pub struct ContainerStyle;

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
