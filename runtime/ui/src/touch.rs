// runtime/ui/src/touch.rs

use egui::{Context, Pos2, Vec2};

use tracing::{info, warn};

/// Represents touchscreen gesture type.
///
/// Current UI philosophy:
/// - kiosk-oriented
/// - simple interaction model
/// - touch-first navigation
#[derive(Debug, Clone, Copy)]
pub enum TouchGesture {
    Tap,
    LongPress,
    Drag,
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
}

/// Represents touch interaction event.
#[derive(Debug, Clone)]
pub struct TouchEvent {
    /// Touchscreen position
    pub position: Pos2,

    /// Gesture classification
    pub gesture: TouchGesture,

    /// Timestamp (milliseconds)
    pub timestamp_ms: u64,
}

impl TouchEvent {
    /// Creates touch event.
    pub fn new(position: Pos2, gesture: TouchGesture, timestamp_ms: u64) -> Self {
        Self {
            position,
            gesture,
            timestamp_ms,
        }
    }
}

/// Responsible for touchscreen interaction handling.
///
/// Responsibilities:
/// - gesture detection
/// - touch routing
/// - kiosk interaction handling
/// - swipe/tap classification
///
/// IMPORTANT:
/// This subsystem DOES NOT:
/// - own orchestration logic
/// - manage runtime behavior
/// - control cluster state
///
/// It only translates user interaction
/// into UI-level events.
pub struct TouchManager {
    /// Last touch position
    last_position: Option<Pos2>,

    /// Last touch timestamp
    last_timestamp_ms: u64,

    /// Drag threshold
    drag_threshold: f32,

    /// Swipe threshold
    swipe_threshold: f32,
}

impl TouchManager {
    /// Creates touch manager.
    pub fn new() -> Self {
        Self {
            last_position: None,

            last_timestamp_ms: 0,

            drag_threshold: 12.0,

            swipe_threshold: 64.0,
        }
    }

    /// Processes touchscreen input frame.
    pub fn update(&mut self, ctx: &Context) -> Option<TouchEvent> {
        let input = ctx.input(|input| input.clone());

        // -------------------------------------------------
        // Pointer/Tap Detection
        // -------------------------------------------------

        if input.pointer.any_pressed() {
            if let Some(position) = input.pointer.interact_pos() {
                let timestamp = current_timestamp_ms();

                self.last_position = Some(position);

                self.last_timestamp_ms = timestamp;

                info!("Touch press detected [{}, {}]", position.x, position.y);

                return Some(TouchEvent::new(position, TouchGesture::Tap, timestamp));
            }
        }

        // -------------------------------------------------
        // Swipe Detection
        // -------------------------------------------------

        if input.pointer.any_released() {
            if let (Some(start), Some(end)) = (self.last_position, input.pointer.interact_pos()) {
                let delta = end - start;

                let timestamp = current_timestamp_ms();

                if delta.length() >= self.swipe_threshold {
                    let gesture = classify_swipe(delta);

                    info!("Swipe gesture detected: {:?}", gesture);

                    return Some(TouchEvent::new(end, gesture, timestamp));
                }

                // -------------------------------------------------
                // Drag Detection
                // -------------------------------------------------

                if delta.length() >= self.drag_threshold {
                    info!("Drag gesture detected");

                    return Some(TouchEvent::new(end, TouchGesture::Drag, timestamp));
                }
            }
        }

        None
    }
}

/// Classifies swipe direction.
fn classify_swipe(delta: Vec2) -> TouchGesture {
    let horizontal = delta.x.abs();

    let vertical = delta.y.abs();

    if horizontal > vertical {
        if delta.x > 0.0 {
            TouchGesture::SwipeRight
        } else {
            TouchGesture::SwipeLeft
        }
    } else {
        if delta.y > 0.0 {
            TouchGesture::SwipeDown
        } else {
            TouchGesture::SwipeUp
        }
    }
}

/// Returns current timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
