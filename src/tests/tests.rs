use chaikin_algorithm::app::algorithm::chaikin::chaikin;
use iced::Point;

/*
Checks if the chaikin function returns the original points when the number of iterations is 0. It creates a vector of 3 points, passes it to the chaikin function with 0 iterations, and asserts that the result is equal to the original points.
*/
#[test]
fn test_chaikin_zero_iterations() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(100.0, 0.0),
        Point::new(100.0, 100.0),
    ];

    let result = chaikin(&points, 0);

    assert_eq!(result, points);
}

/*
Checks the behavior of the chaikin function with multiple iterations.

The test creates a vector of 2 points.
It applies the chaikin function to these points with 1 and 2 iterations, respectively.
It asserts that:
The result with 2 iterations has more points than the result with 1 iteration.
The first and last points of the result with 2 iterations are the same as the original points.

This test verifies that the chaikin function produces more points with more iterations, and that the first and last points remain unchanged.
*/
#[test]
fn test_chaikin_multiple_iterations() {
    let points = vec![Point::new(0.0, 0.0), Point::new(100.0, 0.0)];

    let result_iteration_1 = chaikin(&points, 1);
    let result_iteration_2 = chaikin(&points, 2);

    assert!(result_iteration_1.len() < result_iteration_2.len());

    assert_eq!(result_iteration_2.first(), points.first());
    assert_eq!(result_iteration_2.last(), points.last());
}

/*
Verifies the behavior of the `chaikin` function, which appears to be an implementation of the Chaikin curve algorithm. The test:

Creates a vector of 4 points.
Applies the `chaikin` function to these points with 3 iterations.
Asserts that:
- The resulting vector has more than 4 times the number of points as the original vector.
- The first and last points of the resulting vector are the same as the original points.
- All points in the resulting vector have x-coordinates between 0.0 and 300.0, and y-coordinates between 0.0 and 100.0.
*/
#[test]
fn test_chaikin_integration() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(100.0, 100.0),
        Point::new(200.0, 0.0),
        Point::new(300.0, 100.0),
    ];

    let result = chaikin(&points, 3);

    assert!(result.len() > points.len() * 4);

    assert_eq!(result[0], points[0]);
    assert_eq!(result[result.len() - 1], points[points.len() - 1]);

    for point in &result {
        assert!(point.x >= 0.0 && point.x <= 300.0);
        assert!(point.y >= 0.0 && point.y <= 100.0);
    }
}

/*
Checks the behavior of the `chaikin` function with a single iteration on a simple line. It:

Creates a vector of 2 points representing a line from (0, 0) to (100, 0).
Defines the expected output of the `chaikin` function, which is a vector of 4 points representing the Chaikin curve with 1 iteration.
Calls the `chaikin` function with the input points and 1 iteration.
Asserts that the result is equal to the expected output.

It verifies that the `chaikin` function correctly applies the Chaikin curve algorithm to a simple line with 1 iteration.
*/
#[test]
fn test_chaikin_single_iteration_line() {
    let points = vec![Point::new(0.0, 0.0), Point::new(100.0, 0.0)];

    let expected = vec![
        Point::new(0.0, 0.0),
        Point::new(25.0, 0.0),
        Point::new(75.0, 0.0),
        Point::new(100.0, 0.0),
    ];

    let result = chaikin(&points, 1);

    assert_eq!(result, expected);
}
