use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<Error>> {
    let mut lock_command = Command::new("i3lock").spawn()?;
    thread::sleep(Duration::from_millis(100));
    let mut suspend_command = Command::new("sudo").arg("pm-suspend").spawn()?;
    lock_command.wait()?;
    suspend_command.wait()?;
    Ok(())
}
// i3lock & (sleep 0.1 ; sudo pm-suspend)
