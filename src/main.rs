use anyhow::Result;
use ui::play;

mod ui;
mod game;

fn main() -> Result<()> {
    play()?;

    Ok(())
}
