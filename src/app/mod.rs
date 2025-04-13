mod messages;
pub mod model;
mod update;
pub mod view;
pub mod algorithm;

use iced::{
    self,
    keyboard::{self, key},
    mouse, Command, Element, Length, Subscription,
};
use iced::widget::canvas::{self, Canvas, Event};
use iced::{Application, Rectangle, Renderer, Theme};

pub use messages::Message;
pub use model::Model;

pub struct App {
    model: Model,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    // Initializes the application with the default model state.
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { model: Model::default() }, Command::none())
    }

    // Sets the title of the application window.
    fn title(&self) -> String {
        String::from("Chaikin's Algorithm Animation")
    }

    // Defines the view of the application, rendering the canvas with the model's state.
    fn view(&self) -> Element<Message> {
        Canvas::new(&self.model)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // Updates the application state based on incoming messages.
    fn update(&mut self, message: Message) -> Command<Message> {
        update::update(&mut self.model, message)
    }

    // Subscribes to events, such as time-based ticks for animations.
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

    // Handles canvas-level events, such as mouse clicks, key presses, and cursor movements.
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
            // Handles mouse button presses.
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
            // Handles key presses for actions like starting animations or quitting.
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
            // Handles key releases, such as releasing the Control key.
            Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                if key == Key::Named(key::Named::Control) {
                    self.control_pressed.replace(false);
                    self.dragging.replace(false);
                }
                (canvas::event::Status::Ignored, None)
            }
            // Handles cursor movements for dragging points
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

    // The `draw` function draws the canvas, rendering the current state of the model.
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
