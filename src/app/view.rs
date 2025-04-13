use crate::app::model::Model;
use iced::widget::canvas::{stroke, Frame, LineDash, LineJoin, Path, Stroke, Style, Text};
use iced::{Color, Rectangle, Renderer, Point};

pub fn view(
    renderer: &Renderer,
    model: &Model,
    bounds: Rectangle,
) -> Vec<<Renderer as iced::widget::canvas::Renderer>::Geometry> {
    let mut frame = Frame::new(renderer, bounds.size());

    frame.fill_rectangle(
        Point::ORIGIN,
        bounds.size(),
        Color::BLACK,
    );

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

    // Draw control points
    for point in &model.points {
        let circle = Path::new(|b| b.circle(*point, 2.5));
        frame.fill(&circle, Color::BLACK);
    }

    // Draw Chaikin curve
    if !model.chaikin_points.is_empty() {
        let path = Path::new(|path_builder| {
            path_builder.move_to(model.chaikin_points[0]);
            for point in &model.chaikin_points {
                path_builder.line_to(*point);
            }
        });

        let stroke = Stroke {
            style: Style::Solid(Color::BLACK),
            line_dash: LineDash::default(),
            width: 1.0,
            line_cap: stroke::LineCap::Round,
            line_join: LineJoin::Round,
        };

        frame.stroke(&path, stroke);
    }

    vec![frame.into_geometry()]
}
