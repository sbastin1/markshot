use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

pub fn copy_png_bytes_to_clipboard(image_bytes: &[u8]) -> Result<(), String> {
    let mut child = Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("Failed to start wl-copy: {error}"))?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Failed to open wl-copy stdin".to_string())?;
    stdin
        .write_all(image_bytes)
        .map_err(|error| format!("Failed to write screenshot to wl-copy: {error}"))?;
    drop(stdin);

    thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(())
}
