// examples/manual.rs
// Manual, Windows-only demo for the vfp_messagebox library.
// Run with: cargo run --example manual

#[cfg(windows)]
fn main() {
    use std::ffi::CString;

    println!("Manual test for vfp_messagebox (Windows)");

    unsafe {
        let msg = CString::new(
            "HelloX from Rust! This is the simple message box made in RUST callable from VFP9 SP2\n\nClick OK or close the window.",
        )
        .unwrap();
        let title = CString::new("VFP MessageBox - Simple").unwrap();
        let result = vfp_messagebox::vfp_msgbox_simple(
            msg.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
        );
        println!("Simple result: {} (0=Closed, 1=OK)", result);
    }

    unsafe {
        let msg = CString::new("Do you like Rust?").unwrap();
        let title = CString::new("VFP MessageBox - Yes/No").unwrap();
        let result = vfp_messagebox::vfp_msgbox_yesno(
            msg.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
        );
        println!("Yes/No result: {} (0=Closed, 1=Yes, 2=No)", result);
    }

    unsafe {
        let msg = CString::new("Save changes before exiting?").unwrap();
        let title = CString::new("VFP MessageBox - Yes/No/Cancel").unwrap();
        let result = vfp_messagebox::vfp_msgbox_yesnocancel(
            msg.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
        );
        println!(
            "Yes/No/Cancel result: {} (0=Closed, 1=Yes, 2=No, 3=Cancel)",
            result
        );
    }

    println!("Done.");
}

#[cfg(not(windows))]
fn main() {
    eprintln!("This example only runs on Windows.");
}
