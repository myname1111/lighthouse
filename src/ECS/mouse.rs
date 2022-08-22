use std::time::{Duration, Instant};

use device_query::{DeviceQuery, DeviceState, MouseState};
use nalgebra_glm::*;

/// State of the mouse
#[derive(Clone, Copy)]
pub enum StateOfMouse {
    /// Mouse cannot moved
    Locked(Vec2),
    /// Mouse is free
    Free,
}

impl StateOfMouse {
    /// Switch the values then return. e.g. Locked -> Free and vice versa
    /// on_locked is for when it switches from free -> locked
    pub fn switch(&self, on_locked: Vec2) -> Self {
        match self {
            Self::Locked(_) => Self::Free,
            Self::Free => Self::Locked(on_locked),
        }
    }
}

/// Enum to describe the pressed mouse state
pub enum MousePressed {
    /// Left mouse button is pressed
    LeftMouse,
    /// Right mouse button is pressed
    RightMouse,
    /// Middle mouse button is pressed
    MiddleMouse,
}

/// Mouse wrapper for [MouseState]
///
/// # Example
/// ```rust
/// let mouse = Mouse::new(device.get_mouse, StateOfMouse::Free); // Create new mouse
///
/// // Check if mouse is locked or not
/// match mouse.state {
///     StateOfMouse::Free => println!("Mouse is free!"),
///     StateOfMouse::Locked(Vec2(x, y)) => println!("Mouse is locked into place at {} {}", x, y);
/// }
/// ```
pub struct Mouse {
    /// The Inner mouse
    pub mouse: MouseState,
    /// State of the mouse
    pub state: StateOfMouse,
    /// When was the mouse last pressed
    last_pressed: Instant,
}

impl Mouse {
    /// Creates a new mouse, see [Mouse] to see examples
    ///
    /// # Arguments
    /// mouse: A [MouseState] to be wrapped into [Mouse]
    /// state: The state of the mouse. Is of type [StateOfMouse]
    pub fn new(mouse: MouseState, state: StateOfMouse) -> Self {
        Mouse {
            mouse,
            state,
            last_pressed: Instant::now(),
        }
    }

    /// Returns the what buttons are pressed
    /// 
    /// # Example
    /// 
    pub fn get_pressed(&mut self) -> Vec<MousePressed> {
        let mut out = Vec::new();
        if self.mouse.button_pressed[1] {
            out.push(MousePressed::LeftMouse);
            self.last_pressed = Instant::now();
        }
        if self.mouse.button_pressed[2] {
            out.push(MousePressed::RightMouse);
            self.last_pressed = Instant::now();
        }
        if self.mouse.button_pressed[3] {
            out.push(MousePressed::MiddleMouse);
            self.last_pressed = Instant::now();
        }

        out
    }

    /// Returns the buttons pressed with a cooldown
    pub fn get_pressed_cooldown(&mut self, cooldown: Duration) -> Option<Vec<MousePressed>> {
        if self.last_pressed.elapsed() > cooldown {
            Some(self.get_pressed())
        } else {
            None
        }
    }
}

impl From<DeviceState> for Mouse {
    fn from(device: DeviceState) -> Self {
        Mouse::new(device.get_mouse(), StateOfMouse::Free)
    }
}

impl From<MouseState> for Mouse {
    fn from(device: MouseState) -> Self {
        Mouse::new(device, StateOfMouse::Free)
    }
}
