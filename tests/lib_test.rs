// src/lib.rs
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HDC, HFONT, HMENU, HWND, RECT};
use winapi::um::wingdi::{
    CreateFontW, DeleteObject, GetDeviceCaps, GetTextExtentPoint32W, SelectObject, 
    LOGPIXELSX, LOGPIXELSY, FW_NORMAL, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, 
    CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, VARIABLE_PITCH
};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetDC, GetMessageW, 
    GetSystemMetrics, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, 
    ReleaseDC, ShowWindow, TranslateMessage, UpdateWindow, SetWindowTextW, GetWindowRect,
    SetWindowPos, GetDesktopWindow, SystemParametersInfoW, DrawTextW,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, MB_OK, MSG, 
    SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WM_CLOSE, WM_COMMAND, WM_CREATE, WM_DESTROY, 
    WM_PAINT, WNDCLASSEXW, WS_OVERLAPPED, WS_CAPTION, WS_SYSMENU, WS_VISIBLE, WS_CHILD,
    SPI_GETNONCLIENTMETRICS, NONCLIENTMETRICSW, DT_CENTER, DT_VCENTER, DT_SINGLELINE,
    SWP_NOMOVE, SWP_NOZORDER
};

const ID_BUTTON1: i32 = 1001;
const ID_BUTTON2: i32 = 1002;
const ID_BUTTON3: i32 = 1003;

static mut BUTTON_RESULT: i32 = 0;
static mut MESSAGE_TEXT: Vec<u16> = Vec::new();
static mut BUTTON1_TEXT: Vec<u16> = Vec::new();
static mut BUTTON2_TEXT: Vec<u16> = Vec::new();
static mut BUTTON3_TEXT: Vec<u16> = Vec::new();
static mut BUTTON_COUNT: i32 = 1;

// Convert string to wide string for Windows API
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

// Get system DPI scaling factor
unsafe fn get_dpi_scale() -> f32 {
    let hdc = GetDC(ptr::null_mut());
    let dpi_x = GetDeviceCaps(hdc, LOGPIXELSX);
    ReleaseDC(ptr::null_mut(), hdc);
    dpi_x as f32 / 96.0 // 96 DPI is the standard
}

// Get system font with appropriate scaling
unsafe fn create_scaled_font(base_size: i32) -> HFONT {
    let scale = get_dpi_scale();
    let scaled_size = (base_size as f32 * scale * 1.5) as i32; // Extra scaling for elderly users
    
    // Try to get system font info
    let mut ncm: NONCLIENTMETRICSW = std::mem::zeroed();
    ncm.cbSize = std::mem::size_of::<NONCLIENTMETRICSW>() as u32;
    
    if SystemParametersInfoW(
        SPI_GETNONCLIENTMETRICS,
        ncm.cbSize,
        &mut ncm as *mut _ as *mut _,
        0,
    ) != 0 {
        // Use system message font as base, but increase size
        ncm.lfMessageFont.lfHeight = -scaled_size;
        CreateFontW(
            ncm.lfMessageFont.lfHeight,
            ncm.lfMessageFont.lfWidth,
            ncm.lfMessageFont.lfEscapement,
            ncm.lfMessageFont.lfOrientation,
            ncm.lfMessageFont.lfWeight,
            ncm.lfMessageFont.lfItalic as u32,
            ncm.lfMessageFont.lfUnderline as u32,
            ncm.lfMessageFont.lfStrikeOut as u32,
            ncm.lfMessageFont.lfCharSet as u32,
            ncm.lfMessageFont.lfOutPrecision as u32,
            ncm.lfMessageFont.lfClipPrecision as u32,
            ncm.lfMessageFont.lfQuality as u32,
            ncm.lfMessageFont.lfPitchAndFamily as u32,
            ncm.lfMessageFont.lfFaceName.as_ptr(),
        )
    } else {
        // Fallback to default font
        CreateFontW(
            -scaled_size,
            0,
            0,
            0,
            FW_NORMAL,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            VARIABLE_PITCH,
            to_wide_string("Segoe UI").as_ptr(),
        )
    }
}

// Window procedure
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let scale = get_dpi_scale();
            let button_width = (100.0 * scale * 1.2) as i32;
            let button_height = (35.0 * scale * 1.2) as i32;
            let margin = (20.0 * scale) as i32;
            
            let font = create_scaled_font(12);
            
            // Get window dimensions
            let mut rect: RECT = std::mem::zeroed();
            GetWindowRect(hwnd, &mut rect);
            let window_width = rect.right - rect.left;
            
            // Calculate button positions
            let total_button_width = BUTTON_COUNT * button_width + (BUTTON_COUNT - 1) * margin;
            let start_x = (window_width - total_button_width) / 2;
            let button_y = 120;
            
            // Create buttons based on button count
            if BUTTON_COUNT >= 1 {
                let btn1 = CreateWindowExW(
                    0,
                    to_wide_string("BUTTON").as_ptr(),
                    BUTTON1_TEXT.as_ptr(),
                    WS_VISIBLE | WS_CHILD,
                    start_x,
                    button_y,
                    button_width,
                    button_height,
                    hwnd,
                    ID_BUTTON1 as usize as HMENU,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                winapi::um::winuser::SendMessageW(btn1, winapi::um::winuser::WM_SETFONT, font as WPARAM, 1 as LPARAM);
            }
            
            if BUTTON_COUNT >= 2 {
                let btn2 = CreateWindowExW(
                    0,
                    to_wide_string("BUTTON").as_ptr(),
                    BUTTON2_TEXT.as_ptr(),
                    WS_VISIBLE | WS_CHILD,
                    start_x + button_width + margin,
                    button_y,
                    button_width,
                    button_height,
                    hwnd,
                    ID_BUTTON2 as usize as HMENU,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                winapi::um::winuser::SendMessageW(btn2, winapi::um::winuser::WM_SETFONT, font as WPARAM, 1 as LPARAM);
            }
            
            if BUTTON_COUNT >= 3 {
                let btn3 = CreateWindowExW(
                    0,
                    to_wide_string("BUTTON").as_ptr(),
                    BUTTON3_TEXT.as_ptr(),
                    WS_VISIBLE | WS_CHILD,
                    start_x + 2 * (button_width + margin),
                    button_y,
                    button_width,
                    button_height,
                    hwnd,
                    ID_BUTTON3 as usize as HMENU,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                winapi::um::winuser::SendMessageW(btn3, winapi::um::winuser::WM_SETFONT, font as WPARAM, 1 as LPARAM);
            }
            
            0
        }
        WM_COMMAND => {
            let command_id = (wparam as u32) & 0xFFFF;
            match command_id as i32 {
                ID_BUTTON1 => {
                    BUTTON_RESULT = 1;
                    DestroyWindow(hwnd);
                }
                ID_BUTTON2 => {
                    BUTTON_RESULT = 2;
                    DestroyWindow(hwnd);
                }
                ID_BUTTON3 => {
                    BUTTON_RESULT = 3;
                    DestroyWindow(hwnd);
                }
                _ => {}
            }
            0
        }
        WM_PAINT => {
            let mut ps: winapi::um::winuser::PAINTSTRUCT = std::mem::zeroed();
            let hdc = winapi::um::winuser::BeginPaint(hwnd, &mut ps);
            
            let font = create_scaled_font(14);
            let old_font = SelectObject(hdc, font as *mut _);
            
            let mut rect: RECT = std::mem::zeroed();
            winapi::um::winuser::GetClientRect(hwnd, &mut rect);
            rect.top = 30;
            rect.bottom = 100;
            rect.left += 20;
            rect.right -= 20;
            
            DrawTextW(
                hdc,
                MESSAGE_TEXT.as_ptr(),
                MESSAGE_TEXT.len() as i32 - 1,
                &mut rect,
                DT_CENTER | DT_VCENTER,
            );
            
            SelectObject(hdc, old_font);
            DeleteObject(font as *mut _);
            
            winapi::um::winuser::EndPaint(hwnd, &ps);
            0
        }
        WM_CLOSE => {
            BUTTON_RESULT = 0;
            DestroyWindow(hwnd);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// Export function for VFP to call
#[no_mangle]
pub unsafe extern "C" fn vfp_messagebox(
    message: *const u8,
    title: *const u8,
    button1: *const u8,
    button2: *const u8,
    button3: *const u8,
    button_count: i32,
) -> i32 {
    if message.is_null() || title.is_null() {
        return -1;
    }
    
    // Convert C strings to Rust strings
    let msg_str = std::ffi::CStr::from_ptr(message as *const i8).to_string_lossy();
    let title_str = std::ffi::CStr::from_ptr(title as *const i8).to_string_lossy();
    
    // Store message and title
    MESSAGE_TEXT = to_wide_string(&msg_str);
    let title_wide = to_wide_string(&title_str);
    
    // Handle button texts
    BUTTON_COUNT = button_count.max(1).min(3);
    
    BUTTON1_TEXT = if !button1.is_null() {
        let btn1_str = std::ffi::CStr::from_ptr(button1 as *const i8).to_string_lossy();
        to_wide_string(&btn1_str)
    } else {
        to_wide_string("OK")
    };
    
    BUTTON2_TEXT = if !button2.is_null() && BUTTON_COUNT >= 2 {
        let btn2_str = std::ffi::CStr::from_ptr(button2 as *const i8).to_string_lossy();
        to_wide_string(&btn2_str)
    } else {
        to_wide_string("Cancel")
    };
    
    BUTTON3_TEXT = if !button3.is_null() && BUTTON_COUNT >= 3 {
        let btn3_str = std::ffi::CStr::from_ptr(button3 as *const i8).to_string_lossy();
        to_wide_string(&btn3_str)
    } else {
        to_wide_string("Ignore")
    };
    
    // Register window class
    let class_name = to_wide_string("VFPMessageBox");
    let mut wc: WNDCLASSEXW = std::mem::zeroed();
    wc.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = Some(window_proc);
    wc.hInstance = ptr::null_mut();
    wc.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
    wc.hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
    wc.hbrBackground = (winapi::um::winuser::COLOR_BTNFACE + 1) as HBRUSH;
    wc.lpszClassName = class_name.as_ptr();
    
    RegisterClassExW(&wc);
    
    // Calculate window size based on DPI
    let scale = get_dpi_scale();
    let window_width = (400.0 * scale * 1.3) as i32;
    let window_height = (200.0 * scale * 1.3) as i32;
    
    // Center window on screen
    let screen_width = GetSystemMetrics(SM_CXSCREEN);
    let screen_height = GetSystemMetrics(SM_CYSCREEN);
    let x = (screen_width - window_width) / 2;
    let y = (screen_height - window_height) / 2;
    
    // Create window
    let hwnd = CreateWindowExW(
        0,
        class_name.as_ptr(),
        title_wide.as_ptr(),
        WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU,
        x,
        y,
        window_width,
        window_height,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    
    if hwnd.is_null() {
        return -1;
    }
    
    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);
    
    // Reset result
    BUTTON_RESULT = 0;
    
    // Message loop
    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    
    BUTTON_RESULT
}

// Simple message box with just OK button
#[no_mangle]
pub unsafe extern "C" fn vfp_msgbox_simple(message: *const u8, title: *const u8) -> i32 {
    vfp_messagebox(message, title, ptr::null(), ptr::null(), ptr::null(), 1)
}

// Yes/No message box
#[no_mangle]
pub unsafe extern "C" fn vfp_msgbox_yesno(message: *const u8, title: *const u8) -> i32 {
    let yes_text = std::ffi::CString::new("Yes").unwrap();
    let no_text = std::ffi::CString::new("No").unwrap();
    vfp_messagebox(
        message,
        title,
        yes_text.as_ptr() as *const u8,
        no_text.as_ptr() as *const u8,
        ptr::null(),
        2,
    )
}

// Yes/No/Cancel message box
#[no_mangle]
pub unsafe extern "C" fn vfp_msgbox_yesnocancel(message: *const u8, title: *const u8) -> i32 {
    let yes_text = std::ffi::CString::new("Yes").unwrap();
    let no_text = std::ffi::CString::new("No").unwrap();
    let cancel_text = std::ffi::CString::new("Cancel").unwrap();
    vfp_messagebox(
        message,
        title,
        yes_text.as_ptr() as *const u8,
        no_text.as_ptr() as *const u8,
        cancel_text.as_ptr() as *const u8,
        3,
    )
}

// End of file

// Qodo login notes:
// - To log in using the Qodo CLI (if you have it installed): run `qodo login` in a terminal
//   and follow the interactive prompts. If you need non-interactive auth, set an API token
//   in an environment variable (for example: QODO_API_TOKEN) as described by Qodo docs.
// - To perform login programmatically from Rust: call the Qodo HTTP auth endpoint (per vendor
//   documentation), exchange credentials for a token, then store that token securely (env var,
//   OS credential store, etc.). Do not hardcode credentials in source.
// - Refer to the official Qodo documentation for exact installation and authentication details.