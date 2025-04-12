mod model;
mod update;
mod view;
pub mod algorithm;

use iced::{
    self,
    keyboard::{self, key},
    mouse, Command, Element, Length, Subscription,
};
use iced::widget::canvas::{self, Canvas, Event};
use iced::{Application, Rectangle, Renderer, Theme};

pub struct App {
    model: Model,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { model: Model::default() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Chaikin's Algorithm Animation")
    }

    fn subscription(&self) -> Subscription<Message> {
        if *self.model.run_animation.borrow() {
            iced::time::every(std::time::Duration::from_millis(250)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

impl canvas::Program<Message> for Model {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        use iced::mouse::Button;
        use keyboard::Key;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(Button::Left)) => {
                if *self.control_pressed.borrow() {
                    self.dragging.replace(true);
                    let position = cursor.position_in(bounds).unwrap();
                    let near_point_index = self.points.iter().enumerate().find_map(|(index, &point)| {
                        let distance = (point.x - position.x).hypot(point.y - position.y);
                        if distance <= 5.0 { Some(index) } else { None }
                    });
                    
                    if let Some(index) = near_point_index {
                        self.drag_point.replace(Some((index, self.points[index])));
                        (canvas::event::Status::Captured, None)
                    } else {
                        (canvas::event::Status::Ignored, None)
                    }
                } else if !*self.run_animation.borrow() {
                    let position = cursor.position_in(bounds);
                    (canvas::event::Status::Captured, Some(Message::MousePressed(position.unwrap())))
                } else {
                    (canvas::event::Status::Ignored, None)
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                match key {
                    Key::Named(key::Named::Enter) => (canvas::event::Status::Captured, Some(Message::RunChaikinAnimation)),
                    Key::Named(key::Named::Escape) => (canvas::event::Status::Captured, Some(Message::Quit)),
                    Key::Character(c) if c == "r" => (canvas::event::Status::Captured, Some(Message::Reset)),
                    Key::Named(key::Named::Control) => {
                        self.control_pressed.replace(true);
                        (canvas::event::Status::Ignored, None)
                    }
                    _ => (canvas::event::Status::Ignored, None),
                }
            }
            Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                if key == Key::Named(key::Named::Control) {
                    self.control_pressed.replace(false);
                    self.dragging.replace(false);
                }
                (canvas::event::Status::Ignored, None)
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if *self.dragging.borrow() {
                    let position = cursor.position_in(bounds).unwrap();
                    (canvas::event::Status::Captured, Some(Message::MovePoint(position)))
                } else {
                    (canvas::event::Status::Ignored, None)
                }
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<<Renderer as iced::widget::canvas::Renderer>::Geometry> {
        view::view(renderer, self, bounds)
    }
}
