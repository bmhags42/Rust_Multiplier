use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Container, Row};


pub fn main() -> Result<(), iced::Error> {
    Multiplier::run(Settings::default())
}

pub struct Multiplier {
    first_value: i32,
    second_value: i32,
    product: i32,
    current_view: Views,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FirstIncrementPressed,
    FirstDecrementPressed,
    SecondIncrementPressed,
    SecondDecrementPressed,
    ProductMessage
}

#[derive(Debug, Clone, Copy)]
pub enum Views {
    Multiplier,
}

impl Sandbox for Multiplier {
    type Message = Message;

    fn new() -> Self {
        Multiplier { 
            first_value: 0,
            second_value: 0,
            product: 0,
            current_view: Views::Multiplier,
        }
    }

    fn title(&self) -> String {
        String::from("Multiplier")
    }    

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::FirstIncrementPressed => self.first_value += 1,
            Message::FirstDecrementPressed => self.first_value -= 1,
            Message::SecondIncrementPressed => self.second_value += 1,
            Message::SecondDecrementPressed => self.second_value -= 1,
            Message::ProductMessage => self.product = self.first_value * self.second_value,
        }
    }

    fn view(&self) -> iced::pure::Element<Self::Message> {
        //Items for first row
        let first_label = Text::new(format!("Count: {}", self.first_value));
        let incr_first = Button::new("Increment").on_press(Message::FirstIncrementPressed);
        let decr_first = Button::new("Decrement").on_press(Message::FirstDecrementPressed);

        //Items for second row
        let second_label = Text::new(format!("Count: {}", self.second_value));
        let incr_second = Button::new("Increment").on_press(Message::SecondIncrementPressed);
        let decr_second = Button::new("Decrement").on_press(Message::SecondDecrementPressed);

        //Items for third row (Product row)
        let product_label = Text::new(format!("Product: {}", (self.product)));
        let product_button = Button::new("Get Product").on_press(Message::ProductMessage);

        //Create rows
        let row_first = Row::new().push(incr_first).push(first_label).push(decr_first).spacing(5);
        let row_second = Row::new().push(incr_second).push(second_label).push(decr_second).spacing(5);
        let product_row = Row::new().push(product_button).push(product_label).spacing(5);

        //Create column using row
        let col = Column::new().push(row_first).push(row_second).push(product_row).spacing(5);
        
        //Create container using column
        let multiplayer_layout = Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill);
    
       match self.current_view {
        Views::Multiplier => multiplayer_layout.into()
       }
    }
}