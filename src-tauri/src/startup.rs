use std::sync::Mutex;

pub struct StartupScreenshotRequest(pub Mutex<bool>);

pub fn has_screenshot_arg(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--screenshot")
}
