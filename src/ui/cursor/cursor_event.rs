use crossterm::event::MouseEvent;

#[derive(Debug, PartialEq)]
pub(crate) enum CursorEvent {
    Event(MouseEvent),
    None,
    Stop,
}

impl CursorEvent {
    pub(super) fn position(&self) -> Option<(u16, u16)> {
        match self {
            CursorEvent::Event(mouse_event) => Some((mouse_event.row, mouse_event.column)),
            CursorEvent::None => None,
            CursorEvent::Stop => None,
        }
    }
}
