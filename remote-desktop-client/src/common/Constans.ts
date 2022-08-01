enum MouseStatus {
    MOUSE_DOWN = "mouse-down",
    MOUSE_UP = "mouse-up",
    MOUSE_MOVE = "mouse-move",
    RIGHT_CLICK = "right-click",
}

enum WheelStatus {
    WHEEL_UP = "wheel-up",
    WHEEL_DOWN = "wheel-down",
}

enum KeyboardStatus {
    MOUSE_DOWN = "key-down",
    MOUSE_UP = "key-up",
}

enum MessageType {
    VIDEO_OFFER = "video-offer",
    VIDEO_ANSWER = "video-answer",
    NEW_ICE_CANDIDATE = "new-ice-candidate",
    REMOTE_DESKTOP = "remote-desktop",
    CLOSE_REMOTE_DESKTOP = "close-remote-desktop",
}

enum InputEventType {
    MOUSE_EVENT = "mouse-event",
    KEY_EVENT = "key-event",
}

enum Command {
    MOUSE_EVENT = "mouse_event",
    KEY_EVENT = "key_event",
}

export { MouseStatus, WheelStatus, KeyboardStatus, MessageType, InputEventType, Command }