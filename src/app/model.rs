use std::cell::RefCell;
use iced::Point;

pub struct Model {
    pub points: Vec<Point>,
    pub chaikin_points: Vec<Point>,
    pub iteration: usize,
    pub current_iteration: usize,
    pub run_animation: RefCell<bool>,
    pub control_pressed: RefCell<bool>,
    pub dragging: RefCell<bool>,
    pub drag_point: RefCell<Option<(usize, Point)>>,
    pub show_instructions: RefCell<bool>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            chaikin_points: Vec::new(),
            iteration: 7,
            current_iteration: 0,
            run_animation: RefCell::new(false),
            control_pressed: RefCell::new(false),
            dragging: RefCell::new(false),
            drag_point: RefCell::new(None),
            show_instructions: RefCell::new(true),
        }
    }
}
