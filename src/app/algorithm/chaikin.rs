use iced::Point;

/*======================================================================
This function implements Chaikin's algorithm, a curve-smoothing technique.
The `chaikin` function takes a list of points and a number of iterations as input.
It recursively refines the polyline by introducing new points at 25% and 75% of the distance
between consecutive points, creating a smoother curve with each iteration.
The function ensures the first and last points remain unchanged to preserve the curve's endpoints.*/
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
