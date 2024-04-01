use baseview::MouseCursor as BaseviewCursorIcon;
use vizia_core::prelude::CursorIcon as ViziaCursorIcon;

pub fn cursor_icon_to_cursor_icon(cursor_icon: ViziaCursorIcon) -> Option<BaseviewCursorIcon> {
    match cursor_icon {
        ViziaCursorIcon::Default => Some(BaseviewCursorIcon::Default),
        ViziaCursorIcon::Crosshair => Some(BaseviewCursorIcon::Crosshair),
        ViziaCursorIcon::Hand => Some(BaseviewCursorIcon::Hand),
        ViziaCursorIcon::Arrow => Some(BaseviewCursorIcon::Default),
        ViziaCursorIcon::Move => Some(BaseviewCursorIcon::Move),
        ViziaCursorIcon::Text => Some(BaseviewCursorIcon::Text),
        ViziaCursorIcon::Wait => Some(BaseviewCursorIcon::Working),
        ViziaCursorIcon::Help => Some(BaseviewCursorIcon::Help),
        ViziaCursorIcon::Progress => None,
        ViziaCursorIcon::NotAllowed => Some(BaseviewCursorIcon::NotAllowed),
        ViziaCursorIcon::ContextMenu => None,
        ViziaCursorIcon::Cell => Some(BaseviewCursorIcon::Cell),
        ViziaCursorIcon::VerticalText => Some(BaseviewCursorIcon::VerticalText),
        ViziaCursorIcon::Alias => Some(BaseviewCursorIcon::Alias),
        ViziaCursorIcon::Copy => Some(BaseviewCursorIcon::Copy),
        ViziaCursorIcon::NoDrop => None,
        ViziaCursorIcon::Grab => Some(BaseviewCursorIcon::HandGrabbing),
        ViziaCursorIcon::Grabbing => Some(BaseviewCursorIcon::HandGrabbing),
        ViziaCursorIcon::AllScroll => Some(BaseviewCursorIcon::AllScroll),
        ViziaCursorIcon::ZoomIn => Some(BaseviewCursorIcon::ZoomIn),
        ViziaCursorIcon::ZoomOut => Some(BaseviewCursorIcon::ZoomOut),
        ViziaCursorIcon::EResize => Some(BaseviewCursorIcon::EResize),
        ViziaCursorIcon::NResize => Some(BaseviewCursorIcon::NResize),
        ViziaCursorIcon::NeResize => Some(BaseviewCursorIcon::NeResize),
        ViziaCursorIcon::NwResize => Some(BaseviewCursorIcon::NwResize),
        ViziaCursorIcon::SResize => Some(BaseviewCursorIcon::SResize),
        ViziaCursorIcon::SeResize => Some(BaseviewCursorIcon::SeResize),
        ViziaCursorIcon::SwResize => Some(BaseviewCursorIcon::SwResize),
        ViziaCursorIcon::WResize => Some(BaseviewCursorIcon::WResize),
        ViziaCursorIcon::EwResize => Some(BaseviewCursorIcon::EwResize),
        ViziaCursorIcon::NsResize => Some(BaseviewCursorIcon::NsResize),
        ViziaCursorIcon::NeswResize => Some(BaseviewCursorIcon::NeswResize),
        ViziaCursorIcon::NwseResize => Some(BaseviewCursorIcon::NwseResize),
        ViziaCursorIcon::ColResize => Some(BaseviewCursorIcon::ColResize),
        ViziaCursorIcon::RowResize => Some(BaseviewCursorIcon::RowResize),
        ViziaCursorIcon::None => None,
    }
}