/// This trait is used by the parser to handle extra escape sequences that
/// don't have an impact on the terminal screen directly.
pub trait Callbacks {
    /// This callback is called when the terminal requests an audible bell
    /// (typically with `^G`).
    fn audible_bell(&mut self, _: &mut crate::Screen) {}
    /// This callback is called when the terminal requests a visual bell
    /// (typically with `\eg`).
    fn visual_bell(&mut self, _: &mut crate::Screen) {}
    /// This callback is called when the terminal requests a resize
    /// (typically with `\e[8;<rows>;<cols>t`).
    fn resize(&mut self, _: &mut crate::Screen, _request: (u16, u16)) {}
    /// This callback is called when the terminal requests the window title
    /// to be set (typically with `\e]1;<icon_name>\a`)
    fn set_window_icon_name(
        &mut self,
        _: &mut crate::Screen,
        _icon_name: &[u8],
    ) {
    }
    /// This callback is called when the terminal requests the window title
    /// to be set (typically with `\e]2;<title>\a`)
    fn set_window_title(&mut self, _: &mut crate::Screen, _title: &[u8]) {}
    /// This callback is called when the terminal receives an escape sequence
    /// which is otherwise not implemented.
    fn unhandled_char(&mut self, _: &mut crate::Screen, _c: char) {}
    /// This callback is called when the terminal receives a control
    /// character which is otherwise not implemented.
    fn unhandled_control(&mut self, _: &mut crate::Screen, _b: u8) {}
    /// This callback is called when the terminal receives an escape sequence
    /// which is otherwise not implemented.
    fn unhandled_escape(
        &mut self,
        _: &mut crate::Screen,
        _i1: Option<u8>,
        _i2: Option<u8>,
        _b: u8,
    ) {
    }
    /// This callback is called when the terminal receives a CSI sequence
    /// (`\e[`) which is otherwise not implemented.
    fn unhandled_csi(
        &mut self,
        _: &mut crate::Screen,
        _i1: Option<u8>,
        _i2: Option<u8>,
        _params: &[&[u16]],
        _c: char,
    ) {
    }
    /// This callback is called when the terminal receives a OSC sequence
    /// (`\e]`) which is otherwise not implemented.
    fn unhandled_osc(&mut self, _: &mut crate::Screen, _params: &[&[u8]]) {}
}

impl Callbacks for () {}
