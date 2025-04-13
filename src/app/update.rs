use crate::app::{messages::Message, model::Model};
use crate::app::algorithm::chaikin;
use iced::Command;

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    match message {
        Message::MousePressed(position) => {
            model.points.push(position);
        }
        Message::RunChaikinAnimation => {
            if model.points.is_empty() {
                println!("No points to animate! Add points first.");
                return Command::none();
            }

            if !*model.run_animation.borrow() {
                model.run_animation.replace(true);
            }
        }
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
        Message::Quit => {
            std::process::exit(0);
        }
        Message::Reset => {
            model.points.clear();
            model.chaikin_points.clear();
            model.current_iteration = 0;
            model.run_animation.replace(false);
        }
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
