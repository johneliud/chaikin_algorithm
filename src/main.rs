mod app;
use app::App;
use iced::Application;

pub fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
