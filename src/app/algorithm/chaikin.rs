use iced::Point;

pub fn chaikin(points: &[Point], iterations: usize) -> Vec<Point> {
    if iterations == 0 {
        return points.to_vec();
    }

    let mut new_points = Vec::with_capacity(points.len() * 2);
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
    chaikin(&new_points, iterations - 1)
}
