use crate::app::model::Model;
use iced::widget::canvas::{stroke, Frame, LineDash, LineJoin, Path, Stroke, Style, Text};
use iced::{Color, Rectangle, Renderer, Point};

/* ======================================================================
Renders the application's graphical elements onto the canvas.
# Parameters:
- `renderer`: A reference to the `Renderer` used for drawing.
- `model`: A reference to the application's `Model`, which contains the current state.
- `bounds`: The drawable area within the window.
# Returns:
- A vector of geometries representing the rendered elements.*/
pub fn view(
    renderer: &Renderer,
    model: &Model,
    bounds: Rectangle,
) -> Vec<<Renderer as iced::widget::canvas::Renderer>::Geometry> {
    let mut frame = Frame::new(renderer, bounds.size());

    // Fill the background with black color.
    frame.fill_rectangle(
        Point::ORIGIN,
        bounds.size(),
        Color::BLACK,
    );

    // Display a status message if it exists.
    if !model.status_message.borrow().is_empty() {
        let status_text = Text {
            content: model.status_message.borrow().to_string(),
            position: Point::new(10.0, 20.0),
            color: Color::from_rgb8(255, 0, 0),
            size: iced::Pixels(16.0),
            ..Text::default()
        };
        frame.fill_text(status_text);
    }

    // Draw control points as small circles.
    for point in &model.points {
        let circle = Path::new(|b| b.circle(*point, 5.0));
        frame.fill(&circle, Color::from_rgb8(57, 82, 70));
    }

    // Draw Chaikin curves
    if !model.chaikin_points.is_empty() {
        let path = Path::new(|path_builder| {
            path_builder.move_to(model.chaikin_points[0]);
            for point in &model.chaikin_points {
                path_builder.line_to(*point);
            }
        });

        let stroke = Stroke {
            style: Style::Solid(Color::from_rgb8(57, 82, 70)),
            line_dash: LineDash::default(),
            width: 2.0,
            line_cap: stroke::LineCap::Round,
            line_join: LineJoin::Round,
        };

        frame.stroke(&path, stroke);
    }

    vec![frame.into_geometry()]
}
