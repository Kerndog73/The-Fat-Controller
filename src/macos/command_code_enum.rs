// This file was generated by build.rs

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum CommandCode {
    MouseMoveTo = 0,
    MouseMoveRelative = 1,
    MouseDown = 2,
    MouseUp = 3,
    MouseClick = 4,
    MouseNthClick = 5,
    MouseScrollX = 6,
    MouseScrollY = 7,
    KeyDown = 8,
    KeyUp = 9,
    KeyClick = 10,
}
