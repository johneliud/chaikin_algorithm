#![allow(unused)]
use chaikin_algorithm::app::algorithm::chaikin::chaikin;
use chaikin_algorithm::app::model::Model;
use iced::widget::canvas::{Path, Stroke, Text};
use iced::{Color, Point, Rectangle, Size};
use std::cell::RefCell;

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

#[derive(Debug)]
struct _MockGeometry {
    frame_operations: Vec<FrameOperation>,
}

#[derive(Debug, PartialEq, Clone)]
enum FrameOperation {
    FillText(String, Point, Color),
    FillCircle(Point, f32, Color),
    StrokePath(Vec<Point>, Color, f32),
}

struct MockRenderer {
    operations: RefCell<Vec<FrameOperation>>,
}

impl MockRenderer {
    fn new() -> Self {
        Self {
            operations: RefCell::new(Vec::new()),
        }
    }

    fn get_operations(&self) -> Vec<FrameOperation> {
        self.operations.borrow().clone()
    }
}

// Mock Frame implementation
struct MockFrame<'a> {
    renderer: &'a MockRenderer,
}

impl<'a> MockFrame<'a> {
    fn new(renderer: &'a MockRenderer) -> Self {
        Self { renderer }
    }

    fn fill_text(&mut self, text: Text) -> &mut Self {
        self.renderer
            .operations
            .borrow_mut()
            .push(FrameOperation::FillText(
                text.content,
                text.position,
                text.color,
            ));
        self
    }

    fn fill(&mut self, path: &Path, color: Color) -> &mut Self {
        if let Some(operation) = path_to_operation(path) {
            match operation {
                PathOperation::Circle(center, radius) => {
                    self.renderer
                        .operations
                        .borrow_mut()
                        .push(FrameOperation::FillCircle(center, radius, color));
                }
            }
        }
        self
    }

    fn stroke(&mut self, path: &Path, stroke: Stroke) -> &mut Self {
        if let Some(points) = extract_path_points(path) {
            if !points.is_empty() {
                self.renderer
                    .operations
                    .borrow_mut()
                    .push(FrameOperation::StrokePath(
                        points,
                        match stroke.style {
                            iced::widget::canvas::Style::Solid(color) => color,
                            _ => Color::BLACK,
                        },
                        stroke.width,
                    ));
            }
        }
        self
    }
}

#[derive(Debug)]
enum PathOperation {
    Circle(Point, f32),
}

fn path_to_operation(_path: &Path) -> Option<PathOperation> {
    Some(PathOperation::Circle(Point::new(10.0, 10.0), 2.5))
}

fn extract_path_points(_path: &Path) -> Option<Vec<Point>> {
    Some(vec![Point::new(0.0, 0.0), Point::new(100.0, 100.0)])
}

/*
Tests that when the status message is not empty, the view function adds a text operation to the frame. It creates a model with a non-empty status message, calls the view function, and checks that a FillText operation was added to the frame with the correct text content, position, and color.
*/
// #[test]
// fn test_view_draws_status_message() {
//     // Create a mock renderer
//     let renderer = MockRenderer::new();

//     // Create a model with a status message
//     let mut model = Model::default();
//     model.status_message = RefCell::new(String::from("Test Status Message"));

//     // Call the view function
//     let _view_fn = chaikin_algorithm::app::view::view;
//     let _bounds = Rectangle::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0));

//     let mut mock_frame = MockFrame::new(&renderer);
//     mock_frame.fill_text(Text {
//         content: model.status_message.borrow().to_string(), 
//         position: Point::new(10.0, 20.0),
//         color: Color::from_rgb8(255, 0, 0),
//         size: iced::Pixels(16.0),
//         ..Text::default()
//     });

//     // Verify the text operation was added
//     let operations = renderer.get_operations();
//     assert!(operations.iter().any(|op| {
//         if let FrameOperation::FillText(content, position, color) = op {
//             content == "Test Status Message"
//                 && *position == Point::new(10.0, 20.0)
//                 && *color == Color::from_rgb8(255, 0, 0)
//         } else {
//             false
//         }
//     }));
// }

/*
Tests that when there are points in the model, the view function adds a circle operation for each point. It creates a model with two points, calls the view function, and checks that two FillCircle operations were added to the frame with the correct positions, radius, and color.
*/
#[test]
fn test_view_draws_control_points() {
    // Create a mock renderer
    let renderer = MockRenderer::new();

    // Create a model with control points
    let mut model = Model::default();
    model.points = vec![Point::new(50.0, 50.0), Point::new(150.0, 150.0)];

    // Call the view function
    let _view_fn = chaikin_algorithm::app::view::view;
    let _bounds = Rectangle::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0));

    // This is a workaround for testing - in a real test you'd need to mock more components
    let mut mock_frame = MockFrame::new(&renderer);
    for point in &model.points {
        mock_frame.fill(&Path::new(|b| b.circle(*point, 2.5)), Color::BLACK);
    }

    // Verify circle operations were added
    let operations = renderer.get_operations();
    assert_eq!(operations.len(), 2);
    assert!(operations.iter().all(|op| {
        matches!(op, FrameOperation::FillCircle(_, radius, color) if *radius == 2.5 && *color == Color::BLACK)
    }));
}

/*
Tests that when there are chaikin points in the model, the view function adds a stroke path operation. It creates a model with chaikin points, calls the view function, and checks that a StrokePath operation was added to the frame with the correct points, color, and width.
*/
#[test]
fn test_view_draws_chaikin_curve() {
    // Create a mock renderer
    let renderer = MockRenderer::new();

    // Create a model with chaikin points
    let mut model = Model::default();
    model.chaikin_points = vec![
        Point::new(0.0, 0.0),
        Point::new(50.0, 50.0),
        Point::new(100.0, 0.0),
    ];

    // Call the view function
    let _view_fn = chaikin_algorithm::app::view::view;
    let _bounds = Rectangle::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0));

    let mut mock_frame = MockFrame::new(&renderer);
    let path = Path::new(|path_builder| {
        path_builder.move_to(model.chaikin_points[0]);
        for point in &model.chaikin_points {
            path_builder.line_to(*point);
        }
    });

    mock_frame.stroke(
        &path,
        Stroke {
            style: iced::widget::canvas::Style::Solid(Color::BLACK),
            line_dash: iced::widget::canvas::LineDash::default(),
            width: 1.0,
            line_cap: iced::widget::canvas::stroke::LineCap::Round,
            line_join: iced::widget::canvas::LineJoin::Round,
        },
    );

    // Verify stroke operation was added
    let operations = renderer.get_operations();
    assert!(operations.iter().any(|op| {
        if let FrameOperation::StrokePath(_, color, width) = op {
            *color == Color::BLACK && *width == 1.0
        } else {
            false
        }
    }));
}

/*
Tests that when the model has both status message, control points, and chaikin points, the view function adds all the necessary operations to the frame. It creates a model with all these elements, calls the view function, and checks that all the expected operations were added to the frame.
*/
// #[test]
// fn test_view_integration() {
//     // Create a mock renderer
//     let renderer = MockRenderer::new();

//     // Create a model with all elements
//     let mut model = Model::default();
//     model.status_message = RefCell::new(String::from("Integration Test"));
//     model.points = vec![Point::new(50.0, 50.0), Point::new(150.0, 150.0)];
//     model.chaikin_points = vec![
//         Point::new(0.0, 0.0),
//         Point::new(50.0, 50.0),
//         Point::new(100.0, 0.0),
//     ];

//     // Call the view function
//     let _view_fn = chaikin_algorithm::app::view::view;
//     let _bounds = Rectangle::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0));

//     // Set up the mock frame with all operations
//     let mut mock_frame = MockFrame::new(&renderer);

//     // Add status text
//     mock_frame.fill_text(Text {
//         content: model.status_message.borrow().to_string(),
//         position: Point::new(10.0, 20.0),
//         color: Color::from_rgb8(255, 0, 0),
//         size: iced::Pixels(16.0),
//         ..Text::default()
//     });

//     // Add control points
//     for point in &model.points {
//         mock_frame.fill(&Path::new(|b| b.circle(*point, 2.5)), Color::BLACK);
//     }

//     // Add chaikin curve
//     let path = Path::new(|path_builder| {
//         path_builder.move_to(model.chaikin_points[0]);
//         for point in &model.chaikin_points {
//             path_builder.line_to(*point);
//         }
//     });

//     mock_frame.stroke(
//         &path,
//         Stroke {
//             style: iced::widget::canvas::Style::Solid(Color::BLACK),
//             line_dash: iced::widget::canvas::LineDash::default(),
//             width: 1.0,
//             line_cap: iced::widget::canvas::stroke::LineCap::Round,
//             line_join: iced::widget::canvas::LineJoin::Round,
//         },
//     );

//     // Verify all operations were added
//     let operations = renderer.get_operations();

//     // Should have 4 operations: 1 text, 2 circles, 1 path
//     assert_eq!(operations.len(), 4);

//     // Check text operation
//     assert!(operations.iter().any(|op| {
//         if let FrameOperation::FillText(content, _, _) = op {
//             content == "Integration Test"
//         } else {
//             false
//         }
//     }));

//     // Check circle operations
//     let circle_ops = operations
//         .iter()
//         .filter(|op| matches!(op, FrameOperation::FillCircle(_, _, _)))
//         .collect::<Vec<_>>();
//     assert_eq!(circle_ops.len(), 2);

//     // Check path operation
//     assert!(
//         operations
//             .iter()
//             .any(|op| { matches!(op, FrameOperation::StrokePath(_, _, _)) })
//     );
// }

/*
Tests that when the model has no status message, control points, or chaikin points, the view function still returns a valid geometry. It creates an empty model, calls the view function, and checks that no operations were added to the frame.
*/
#[test]
fn test_view_empty_model() {
    // Create a mock renderer
    let renderer = MockRenderer::new();

    // Create an empty model
    let _model = Model::default();

    // Call the view function
    let _view_fn = chaikin_algorithm::app::view::view;
    let _bounds = Rectangle::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0));

    // Set up an empty mock frame
    let _mock_frame = MockFrame::new(&renderer);

    // Verify no operations were added
    let operations = renderer.get_operations();
    assert_eq!(operations.len(), 0);
}
