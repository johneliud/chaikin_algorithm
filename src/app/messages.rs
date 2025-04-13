use iced::Point;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RunChaikinAnimation,
    Tick,
    MousePressed(Point),
    Reset,
    Quit,
    MovePoint(Point),
}
