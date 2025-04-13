use iced::Point;

#[derive(Debug, Clone, Copy)]
/*===============================================================
This file defines the `Message` enum, which represents the various events and actions
that can occur within the application. These messages are used to handle user interactions
and application state updates, such as running the Chaikin animation, handling mouse input,
resetting the canvas, quitting the application, or moving a point in real time.*/
pub enum Message {
    RunChaikinAnimation,
    Tick,
    MousePressed(Point),
    Reset,
    Quit,
    MovePoint(Point),
}
