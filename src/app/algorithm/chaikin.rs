// First version: Single iteration only
use iced::Point;

pub fn chaikin(points: &[Point]) -> Vec<Point> {
    let mut new_points = Vec::new();
    if points.is_empty() {
        return new_points;
    }

    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];

        new_points.push(Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        });
        new_points.push(Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        });
    }

    new_points.push(*points.last().unwrap());
    new_points
}
