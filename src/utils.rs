#[allow(dead_code)]
pub fn parse_u64(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}

#[cfg(windows)]
pub fn enable_ansi_support() {
    type HANDLE = *mut std::ffi::c_void;
    type DWORD = u32;
    type BOOL = i32;

    const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
    const STD_ERROR_HANDLE: DWORD = -12i32 as DWORD;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
    const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

    unsafe extern "system" {
        fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
        fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut DWORD) -> BOOL;
        fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
    }

    unsafe {
        for std_handle in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
            let handle = GetStdHandle(std_handle);
            if !handle.is_null() && handle != INVALID_HANDLE_VALUE {
                let mut mode: DWORD = 0;
                if GetConsoleMode(handle, &mut mode) != 0 {
                    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                    SetConsoleMode(handle, mode);
                }
            }
        }
    }
}

#[cfg(not(windows))]
pub fn enable_ansi_support() {}

pub fn clear_screen() {
    #[cfg(windows)]
    {
        if win32_clear_screen() {
            return;
        }
    }
    ansi_clear_screen();
}

fn ansi_clear_screen() {
    use std::io::{self, Write};
    print!("\x1B[2J\x1B[3J\x1B[H");
    let _ = io::stdout().flush();
}

#[cfg(windows)]
#[allow(non_camel_case_types, non_snake_case)]
fn win32_clear_screen() -> bool {
    type HANDLE = *mut std::ffi::c_void;
    type DWORD = u32;
    type WORD = u16;
    type BOOL = i32;
    type WCHAR = u16;

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct COORD {
        x: i16,
        y: i16,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct SMALL_RECT {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD,
        dwCursorPosition: COORD,
        wAttributes: WORD,
        srWindow: SMALL_RECT,
        dwMaximumWindowSize: COORD,
    }

    const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
    const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

    unsafe extern "system" {
        fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
        fn GetConsoleScreenBufferInfo(
            hConsoleOutput: HANDLE,
            lpConsoleScreenBufferInfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
        ) -> BOOL;
        fn FillConsoleOutputCharacterW(
            hConsoleOutput: HANDLE,
            cCharacter: WCHAR,
            nLength: DWORD,
            dwWriteCoord: COORD,
            lpNumberOfCharsWritten: *mut DWORD,
        ) -> BOOL;
        fn FillConsoleOutputAttribute(
            hConsoleOutput: HANDLE,
            wAttribute: WORD,
            nLength: DWORD,
            dwWriteCoord: COORD,
            lpNumberOfAttrsWritten: *mut DWORD,
        ) -> BOOL;
        fn SetConsoleCursorPosition(hConsoleOutput: HANDLE, dwCursorPosition: COORD) -> BOOL;
    }

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return false;
        }

        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD { x: 0, y: 0 },
            dwCursorPosition: COORD { x: 0, y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT { left: 0, top: 0, right: 0, bottom: 0 },
            dwMaximumWindowSize: COORD { x: 0, y: 0 },
        };

        if GetConsoleScreenBufferInfo(handle, &mut csbi) == 0 {
            return false;
        }

        let cell_count = csbi.dwSize.x as DWORD * csbi.dwSize.y as DWORD;
        let home = COORD { x: 0, y: 0 };
        let mut written: DWORD = 0;

        if FillConsoleOutputCharacterW(handle, ' ' as WCHAR, cell_count, home, &mut written) == 0 {
            return false;
        }

        if FillConsoleOutputAttribute(handle, csbi.wAttributes, cell_count, home, &mut written) == 0 {
            return false;
        }

        if SetConsoleCursorPosition(handle, home) == 0 {
            return false;
        }
    }

    true
}


