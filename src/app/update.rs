use crate::app::{messages::Message, model::Model};
use crate::app::algorithm::chaikin;
use iced::Command;

/* ===================================================================
Updates the application state based on the received `Message`.

# Parameters:
- `model`: A mutable reference to the application's `Model`, representing its current state.
- `message`: The `Message` enum variant that represents the event or action to process.
# Returns:
- A `Command<Message>` that can be used to trigger asynchronous actions or return no action (`Command::none()`).*/
pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    match message {
        // Adds a new point to the canvas when the mouse is pressed.
        Message::MousePressed(position) => {
            model.points.push(position);
            model.status_message.replace(String::new());
        }
        // Starts the Chaikin animation if there are points to animate.
        Message::RunChaikinAnimation => {
            if model.points.is_empty() {
                model.status_message.replace("No points to animate! Add points first.".to_string());
                return Command::none();
            }

            model.status_message.replace(String::new());
            if !*model.run_animation.borrow() {
                model.run_animation.replace(true);
            }
        }
        // Handles animation ticks, updating the Chaikin-smoothed points for each iteration.
        Message::Tick => {
            if *model.run_animation.borrow() {
                if model.current_iteration < model.iteration && model.points.len() > 1 {
                    if model.points.len() == 2 {
                        model.chaikin_points = model.points.clone();
                        model.current_iteration = 0;
                    } else {
                        model.chaikin_points = chaikin(&model.points, model.current_iteration);
                        model.current_iteration += 1;
                    }
                } else {
                    if model.points.is_empty() {
                        model.chaikin_points.clear();
                    }
                    model.current_iteration = 0;
                }
            }
        }
        // Exits the application.
        Message::Quit => {
            std::process::exit(0);
        }
        // Resets the canvas, clearing all points and resetting the animation state.
        Message::Reset => {
            model.points.clear();
            model.chaikin_points.clear();
            model.current_iteration = 0;
            model.run_animation.replace(false);
            model.status_message.replace("Set a new object.".to_string());
        }
        // Updates the position of a point being dragged in real time.
        Message::MovePoint(position) => {
            if *model.dragging.borrow() {
                if let Some((index, _)) = model.drag_point.borrow().as_ref() {
                    model.points[*index] = position;
                }
            }
        }
    }

    Command::none()
}
