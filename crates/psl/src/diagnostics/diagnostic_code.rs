pub enum DiagnosticsCode {
    Error(ErrorCode),
}

#[repr(u32)]
pub enum ErrorCode {
    Syntax = 1,
}
