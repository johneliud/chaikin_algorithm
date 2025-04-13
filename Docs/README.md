# Chaikin's Algorithm Animation Project: An Exquisite Implementation of a Timeless Concept

## Introduction

*In the realm of computer graphics, the pursuit of creating smooth, aesthetically pleasing curves from a set of points or a polyline has been a longstanding challenge. Chaikin's algorithm, a seminal method devised to address this issue, has been a cornerstone in the field, renowned for its efficacy in refining and smoothing curves. This project undertakes the ambitious task of not only implementing Chaikin's algorithm but also presenting it in a visually captivating animation, thereby providing an immersive experience for the user.*

## Project Overview

The project aims to provide a comprehensive and immersive experience for users by implementing Chaikin's algorithm and presenting it in an animation. By combining the algorithm's elegance with the visual appeal of an animation, the project offers a unique and engaging way to explore the concept of curve smoothing. The instrutions for the proect can be found [here](INSTRUCTIONS.md).
At its core, Chaikin's algorithm operates by iteratively introducing new points along each line segment at 25% and 75% of the distance between the original points. This process, repeated over multiple iterations, yields a curve of remarkable smoothness, effectively mitigating the angularity of the original polyline. The algorithm's versatility and efficiency make it an indispensable tool in various applications, from graphic design to engineering, where the creation of smooth curves is paramount.

## Table of Contents

- [Chaikin's Algorithm Animation Project: An Exquisite Implementation of a Timeless Concept](#chaikins-algorithm-animation-project-an-exquisite-implementation-of-a-timeless-concept)
  - [Introduction](#introduction)
  - [Project Overview](#project-overview)
  - [Table of Contents](#table-of-contents)
  - [Key Features and Functionalities](#key-features-and-functionalities)
  - [Technical Insights and Implementation Details](#technical-insights-and-implementation-details)
  - [Functionality](#functionality)
    - [Code Structure](#code-structure)
    - [Implementation Details](#implementation-details)
  - [Installation and Execution](#installation-and-execution)
  - [Technology Stack](#technology-stack)
  - [Testing](#testing)
  - [Conclusion](#conclusion)

## Key Features and Functionalities

`User Interaction`: *The program is designed to be highly interactive, allowing users to draw one or more points on a canvas using the left mouse button. This intuitive interface enables users to explore the algorithm's capabilities in a hands-on manner.*

`Animation`: Upon pressing the Enter key, the program initiates an animation that vividly illustrates each step of Chaikin's algorithm. This visual representation not only enhances understanding but also provides a mesmerizing display of how the algorithm progressively refines the curve.

`Control and Navigation`: The application includes a range of controls to enhance user experience. Pressing Escape allows users to exit the application, while pressing r resets the canvas, enabling users to start anew. Furthermore, holding down the Control key and dragging the mouse facilitates the real-time movement of control points, offering unparalleled flexibility and precision.
Installation and Execution: For those interested in exploring the project's inner workings or contributing to its development, detailed installation instructions are provided. By cloning the repository and executing the cargo run command, users can delve into the world of Chaikin's algorithm and experience the project firsthand.

## Technical Insights and Implementation Details

The implementation of Chaikin's algorithm in this project is a testament to the power of meticulous design and coding. By leveraging the capabilities of modern programming languages and development tools, the project achieves a balance between complexity and simplicity, making it accessible to a wide range of users, from novice programmers to seasoned developers.

## Functionality

- The program allows users to:

  - Draw one or more points on a canvas using the left mouse button.
  - Press Enter to start the animation, which illustrates each step of Chaikin's algorithm.
  - Press Escape to quit the application.
  - Press r to reset the canvas and draw new points.
  - Hold down the Control key and drag the mouse to move a point in real time.

### Code Structure

The project is structured into the following modules:

`app`: Contains the main application logic, including the Model and View components.
`model`: Defines the data structures and algorithms used in the application, including the implementation of Chaikin's algorithm.
`view`: Handles the rendering of the canvas and the animation.
Key Components
`src/main.rs`: The entry point of the application, responsible for setting up the application and running the main loop.
`src/lib.rs`: Defines the library module, which contains the app module.
`src/app/view.rs:` Implements the view component, responsible for rendering the canvas and the animation.

### Implementation Details

The implementation of Chaikin's algorithm is as follows:

The model module defines a Model struct, which contains a vector of points and a vector of Chaikin points.
The view module defines a view function, which takes a Model instance and a Renderer instance as input, and returns a vector of geometries to be rendered.
The view function uses the Path and Stroke components from the iced library to render the curve and the control points.

- Example Use Cases
  - Drawing a smooth curve using Chaikin's algorithm:

```rs
let mut model = Model::new();
model.add_point(Point::new(10.0, 10.0));
model.add_point(Point::new(20.0, 20.0));
model.add_point(Point::new(30.0, 30.0));
let chaikin_points = model.chaikin_points();

```

- Animating the curve:

```rs
let mut model = Model::new();
model.add_point(Point::new(10.0, 10.0));
model.add_point(Point::new(20.0, 20.0));
model.add_point(Point::new(30.0, 30.0));
let chaikin_points = model.chaikin_points();
```

## Installation and Execution

- To instal and run the project, follow these steps:

```sh
git clone https://github.com/johneliud/chaikin_algorithm.git
cd chaikin_algorithm
cargo run
```

## Technology Stack

Some of the technology stack used in the Chaikin's Algorithm Animation project includes:

- `Front-end:` Rust, Iced framework
- `Back-end:` None (since it's a client-side application)
- `Infrastructure:` Cargo (Rust package manager)
- `Tools and Libraries:` Git (version control), Clippy (Rust linter)

## Testing

To ensure the correctness of the Chaikin's algorithm implementation, we will use the iced testing framework. The testing framework will consist of the following components:

- `tests.rs:` A test module that contains unit tests for the Model and View components
- `test_utils.rs:` A utility module that provides helper functions for testing

- Run the tests using the command:

```sh
cargo test
```

This will execute the test cases and report any failures or errors.

## Conclusion

The Chaikin's Algorithm Animation project stands as a shining example of how complex mathematical concepts can be transformed into engaging, interactive experiences. Through its rich interface, detailed animation, and user-centric design, this project not only educates but also inspires, inviting users to explore the fascinating world of computer graphics and algorithmic design. Whether you are a student seeking to understand the intricacies of Chaikin's algorithm, a developer looking to integrate smooth curve generation into your projects, or simply an enthusiast of computer graphics, this project is sure to captivate and enlighten.
