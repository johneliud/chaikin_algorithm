# Chaikin's Algorithm Animation Project: An Exquisite Implementation of a Timeless Concept

## Introduction

_In the realm of computer graphics, the pursuit of creating smooth, aesthetically pleasing curves from a set of points or a polyline has been a longstanding challenge._

_Chaikin's algorithm, a seminal method devised to address this issue, has been a cornerstone in the field, renowned for its efficacy in refining and smoothing curves. This project undertakes the ambitious task of not only implementing Chaikin's algorithm but also presenting it in a visually captivating animation, thereby providing an immersive experience for the user._

## Project Overview

_The project aims to provide a comprehensive and immersive experience for users by implementing Chaikin's algorithm and presenting it in an animation. By combining the algorithm's elegance with the visual appeal of an animation, the project offers a unique and engaging way to explore the concept of curve smoothing. The instrutions for the proect can be found [here](INSTRUCTIONS.md).
At its core, Chaikin's algorithm operates by iteratively introducing new points along each line segment at 25% and 75% of the distance between the original points. This process, repeated over multiple iterations, yields a curve of remarkable smoothness, effectively mitigating the angularity of the original polyline. The algorithm's versatility and efficiency make it an indispensable tool in various applications, from graphic design to engineering, where the creation of smooth curves is paramount._

## Table of Contents

- [Chaikin's Algorithm Animation Project: An Exquisite Implementation of a Timeless Concept](#chaikins-algorithm-animation-project-an-exquisite-implementation-of-a-timeless-concept)
  - [Introduction](#introduction)
  - [Project Overview](#project-overview)
  - [Table of Contents](#table-of-contents)
  - [Installation and Execution](#installation-and-execution)
  - [User Interation](#user-interation)
    - [Creating a point](#creating-a-point)
    - [Animation](#animation)
    - [Control and Navigation](#control-and-navigation)
    - [Clearing the Screen](#clearing-the-screen)
  - [Implementation Details (How it Works)](#implementation-details-how-it-works)
    - [Quick Basic controls](#quick-basic-controls)
    - [📁 Project Structure](#-project-structure)
    - [Code Venture](#code-venture)
      - [📄 main.rs](#-mainrs)
      - [📄 lib.rs](#-librs)
      - [📄 src/app/mod.rs](#-srcappmodrs)
      - [📄 src/app/messages.rs](#-srcappmessagesrs)
      - [📄 src/app/model.rs](#-srcappmodelrs)
        - [🧠 Field Overview](#-field-overview)
      - [📄 src/app/update.rs](#-srcappupdaters)
      - [📄 src/app/view.rs](#-srcappviewrs)
  - [Technology Stack](#technology-stack)
  - [Testing](#testing)
  - [Conclusion](#conclusion)

## Installation and Execution

_For those interested in exploring the project's inner workings or contributing to its development, detailed installation instructions are provided._

- _To instal and run the project, follow these steps:_

```sh
git clone https://github.com/johneliud/chaikin_algorithm.git
cd chaikin_algorithm
cargo run
```

---

## User Interation

### Creating a point

_The program is designed to be highly interactive, allowing users to draw one or more points on a canvas using the left mouse button. This intuitive interface enables users to explore the algorithm's capabilities in a hands-on manner._

- _The user can create as much points on the canvas as desired._

### Animation

_Upon pressing the, the `Enter` key, the program initiates an animation that vividly illustrates each step of Chaikin's algorithm. This visual representation not only enhances understanding but also provides a mesmerizing display of how the algorithm progressively refines the curve._

### Control and Navigation

_The application includes a range of controls. Pressing `Escape` allows users to exit the application and minimize the windowFurthermore, holding down the `Control key` and dragging the point in a certain direction, facilitates the real-time movement of control points, offering unparalleled flexibility and precision, making the focused-point to be manipulated in real-time rational to the curve._

### Clearing the Screen

_Pressing `r` key resets/clears the canvas, enabling users to start anew._

---

## Implementation Details (How it Works)

_The implementation of Chaikin's algorithm in this project is a testament to the power of meticulous design and coding. By leveraging the capabilities of modern programming languages like the Rust Progrmming language, the project achieves a balance between complexity and simplicity, making it accessible to a wide range of users, from novice programmers to seasoned developers._

### Quick Basic controls

- Draw one or more points on a canvas using the left mouse button.
- Press `Enter` to start the animation, which illustrates each step of Chaikin's algorithm.
- Press `Escape` to quit the application.
- Press `r` to reset the canvas and draw new points.
- Hold down the `Control` key and drag the mouse to move a point in real time.

<hr style="border: none solid #ccc; margin: 20px 0;"/>

### 📁 Project Structure

_This document outlines the purpose and organization of each file and directory in the project._

```sh
.
├── Docs/
│    ├── INSTRUCTIONS.md
│    ├── LICENSE.md
│    └── README.md
│
├── src/
│     ├── app/
│     │   ├── mod.rs
│     │   ├── messages.rs
│     │   ├── model.rs
│     │   ├── update.rs
│     │   ├── view.rs
│     │   └── algorithm/
│     │       ├── mod.rs
│     │       └── chaikin.rs
│     ├──tests/
│     │   └── tests.rs
│     ├── lib.rs
│     └──main.rs
├── .gitignore
├── Cargo.lock
└── Cargo.toml

```

### Code Venture

#### 📄 main.rs

_This is the entry point of a GUI application built with the `iced framework`, a cross-platform GUI library for Rust inspired by Elm._

```rs
mod app;
use app::App;
use iced::Application;

pub fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
```

_This file content will launch the GUI application with the default setting configuration (like window size, theme, flags)._
_The function returns a vslid resoponse, and check for errors if anyt from the implied crate_

#### 📄 lib.rs

_Declares the app module and makes it public—accessible from other modules or crates (like tests, or when used as a dependency)._

- _It is the entry point of the other crates and tests_

#### 📄 src/app/mod.rs

**1** 🧠 **Application Lifecycle** – `impl Application for App`

- _Implements the iced::Application trait, defining how the app starts, updates, renders, and reacts to user input_

| Method           | Purpose                                                     |
| ---------------- | ----------------------------------------------------------- |
| `new()`          | Initializes the app with default state.                     |
| `title()`        | Sets window title: `"Chaikin's Algorithm Animation"`        |
| `view()`         | Renders the app by drawing the model on a full-size canvas. |
| `update()`       | Reacts to messages (like user inputs or ticks).             |
| `subscription()` | Triggers regular updates if animation is running.           |

**2**🎨 **Canvas Interactivity** – `impl canvas::Program`

- _This part gives the model direct access to canvas interactivity, like mouse clicks, key presses, or drawing._

| Method     | Purpose                                                      |
| ---------- | ------------------------------------------------------------ |
| `update()` | Reacts to canvas-level events like mouse or keyboard input.  |
| `draw()`   | Delegates drawing to the `view` module using the `Renderer`. |

**3**🖱 **Event Highlights**

- _Mouse Click: Adds point or selects point if Control is pressed._

- _Keyboard:_

  - `Enter`: Starts animation.

  - `Escape`: Quits.

  - `r`: Resets.

  - `Ctrl`: Enables drag mode.

- _**Cursor Move**: Moves a selected point while dragging._

**4**
🔗 **Integration**

- _This module file is the bridge between user input, state management, and graphical rendering. It enables:_

  - Smooth animations via Subscription.

  - Interactive drawing/editing of points.

  - Application flow managed through declarative Messages.

#### 📄 src/app/messages.rs

- _The file contains the Messages enum, with action variants, which defines the different types of messages that can be sent to the app._
- _These messages update the state of the app, in every instance called. The update happens in the `src/app/update.rs` file._
- _Here is wat each variant does:_

| Variant               | Purpose                                                            |
| --------------------- | ------------------------------------------------------------------ |
| `RunChaikinAnimation` | Triggered when user presses `Enter` to start Chaikin's animation.  |
| `Tick`                | Fired periodically during animation to proceed to the next frame.  |
| `MousePressed(Point)` | Called when user clicks on the canvas to place a point.            |
| `Reset`               | Resets the state (e.g., clears points).                            |
| `Quit`                | Terminates the application.                                        |
| `MovePoint(Point)`    | Moves a selected point (when dragging with Control key held down). |

#### 📄 src/app/model.rs

- _This file defines the core Model struct that holds the entire application state for the Chaikin's curve animation and user interactions._

##### 🧠 Field Overview

| Field               | Type                            | Purpose                                                  |
| ------------------- | ------------------------------- | -------------------------------------------------------- |
| `points`            | `Vec<Point>`                    | Stores user-defined input points.                        |
| `chaikin_points`    | `Vec<Point>`                    | Stores points generated by Chaikin’s algorithm.          |
| `iteration`         | `usize`                         | Max number of iterations for curve refinement.           |
| `current_iteration` | `usize`                         | Current iteration step during animation.                 |
| `run_animation`     | `RefCell<bool>`                 | Indicates if animation is running.                       |
| `control_pressed`   | `RefCell<bool>`                 | Tracks whether Ctrl key is held (for dragging behavior). |
| `dragging`          | `RefCell<bool>`                 | True if a point is being dragged.                        |
| `drag_point`        | `RefCell<Option<(usize, Point)` | Stores index and coordinates of the point being dragged. |

#### 📄 src/app/update.rs

| Message Variant          | Behavior                                                                                                                                |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| `MousePressed(position)` | Adds the clicked point (`position`) to `model.points`.                                                                                  |
| `RunChaikinAnimation`    | Starts the animation by setting `run_animation` to `true`, only if it isn’t already running.                                            |
| `Tick`                   | Drives the animation loop. If conditions are met, computes Chaikin points iteratively.                                                  |
|                          | - If only two points exist, they are cloned directly.                                                                                   |
|                          | - Otherwise, applies Chaikin algorithm using current iteration.                                                                         |
|                          | - Stops generating new Chaikin points if the maximum number of iterations has been reached or if there aren’t enough points to process. |
| `Quit`                   | Terminates the application.                                                                                                             |
| `Reset`                  | Clears all points, Chaikin points, resets iteration, and stops the animation.                                                           |
| `MovePoint(position)`    | If dragging is active, updates the position of the dragged point in `model.points`.                                                     |

#### 📄 src/app/view.rs

- _Below is a list of components in the view.rs file, along with their respective functions and purposes:_

| Component                             | Description                                                            |
| ------------------------------------- | ---------------------------------------------------------------------- |
| `view` function                       | Responsible for rendering the model’s current state onto the canvas.   |
| `renderer: &Renderer`                 | Reference to the renderer used for drawing.                            |
| `model: &Model`                       | Provides access to points and Chaikin-smoothed points.                 |
| `bounds: Rectangle`                   | The drawable area within the window.                                   |
| `Frame::new(...)`                     | Initializes a new canvas drawing frame.                                |
| `Path::new(...circle...)`             | Draws small circles for each raw point in `model.points`.              |
| `Color::BLACK`                        | The color used for point fill and smoothed line stroke.                |
| `if !model.chaikin_points.is_empty()` | Conditionally draws the smoothed line only if points exist.            |
| `path_builder.move_to(...)`           | Starts drawing from the first Chaikin point.                           |
| `path_builder.line_to(...)`           | Draws straight lines through all smoothed points.                      |
| `Stroke` object                       | Defines stroke style, width, and line caps/joins for the Chaikin path. |
| `frame.stroke(...)`                   | Renders the Chaikin path with the defined stroke style.                |
| `frame.into_geometry()`               | Converts the frame into drawable geometry for the canvas.              |

## Technology Stack

Some of the technology stack used in the Chaikin's Algorithm Animation project includes:

- `Front-end:` Rust, Iced framework
- `Back-end:` None (since it's a client-side application)
- `Infrastructure:` Cargo (Rust package manager)
- `Tools and Libraries:` Git (version control), Clippy (Rust linter)

## Testing

To ensure the correctness of the Chaikin's algorithm implementation, we will use the iced testing framework. The testing framework will consist of the following components:

- `tests.rs:` A test module that contains unit tests for the Model and View components

- Run the tests using the command:

```sh
cargo test
```

This will execute the test cases and report any failures or errors.

## Conclusion

The Chaikin's Algorithm Animation project stands as a shining example of how complex mathematical concepts can be transformed into engaging, interactive experiences. Through its rich interface, detailed animation, and user-centric design, this project not only educates but also inspires, inviting users to explore the fascinating world of computer graphics and algorithmic design. Whether you are a student seeking to understand the intricacies of Chaikin's algorithm, a developer looking to integrate smooth curve generation into your projects, or simply an enthusiast of computer graphics, this project is sure to captivate and enlighten.
