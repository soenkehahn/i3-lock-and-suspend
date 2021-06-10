use cradle::*;
use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let Exit(_) = cmd_result!(%"nmcli connection down", "Pixel 2 Network")?;
    let mut lock_command = Command::new("i3lock").args(vec!["-c", "222222"]).spawn()?;
    thread::sleep(Duration::from_millis(200));
    let mut suspend_command = Command::new("sudo").arg("pm-suspend").spawn()?;
    lock_command.wait()?;
    suspend_command.wait()?;
    Ok(())
}
