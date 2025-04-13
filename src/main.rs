mod app;
use app::App;
use iced::Application;

/* =========================================================
The `main` function initializes the application and runs it with default settings.
# Returns:
- An `iced::Result` indicating success or failure during application execution.*/
pub fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
