use crate::constants::*;

pub fn term_width() -> usize {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        #[repr(C)]
        struct Winsize {
            rows: u16,
            cols: u16,
            xpix: u16,
            ypix: u16,
        }
        let mut ws = Winsize { rows: 0, cols: 0, xpix: 0, ypix: 0 };
        #[cfg(target_os = "macos")]
        const TIOCGWINSZ: u64 = 0x40087468;
        #[cfg(not(target_os = "macos"))]
        const TIOCGWINSZ: u64 = 0x5413;
        let fd = std::io::stderr().as_raw_fd();
        let ok = unsafe { libc::ioctl(fd, TIOCGWINSZ, &mut ws) };
        if ok == 0 && ws.cols > 0 {
            return ws.cols as usize;
        }
    }
    80
}

pub fn rule(ch: &str, w: usize) {
    let inner = w.saturating_sub(2);
    println!("  {DIM}{}{RESET}", ch.repeat(inner));
}

pub fn banner(w: usize, version: &str) {
    let inner = w.saturating_sub(4);
    let title = "A S M L I N G S";
    let sub = "x86 · 16-bit assembly exercises";
    let ver_tag = format!("v{version}");

    let left = format!("  {title}  ·  {sub}  ");
    let right = format!("  {ver_tag}  ");
    let pad = inner.saturating_sub(left.len() + right.len());

    println!();
    println!("  {BOLD}┌{}┐{RESET}", "─".repeat(inner));
    println!(
        "  {BOLD}│{RESET}{BOLD}{left}{RESET}{DIM}{}{right}{RESET}{BOLD}│{RESET}",
        " ".repeat(pad)
    );
    println!("  {BOLD}└{}┘{RESET}", "─".repeat(inner));
    println!();
}

pub fn progress_bar(current: usize, total: usize, w: usize) {
    let label = format!("  {} / {}  ", current, total);
    let bar_w = w.saturating_sub(2 + label.len());
    let filled = if total == 0 { 0 } else { current * bar_w / total };
    let empty = bar_w.saturating_sub(filled);

    println!(
        "  {GREEN}{}{RESET}{DIM}{}{RESET}{DIM}{label}{RESET}",
        "█".repeat(filled),
        "░".repeat(empty),
    );
}
