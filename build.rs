use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("assets/branding/icon_win_bitmato_chess.ico")
            .compile()?
    }

    Ok(())
}