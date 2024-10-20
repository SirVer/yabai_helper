mod schema;

use anyhow::Result;
use argh::FromArgs;

use schema::*;
use std::process::Command;

#[derive(FromArgs, Debug)]
/// Helper tool to augment yabai to my needs.
struct Args {
    #[argh(subcommand)]
    cmd: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommand {
    FocusNext(FocusNextArgs),
    FocusPrev(FocusPrevArgs),
    ToggleSpaceLayout(ToggleSpaceLayoutArgs),
    FocusNextDisplay(FocusNextDisplayArgs),
}

#[derive(FromArgs, Debug)]
/// Focus the next window
#[argh(subcommand, name = "focus-next")]
struct FocusNextArgs {}

#[derive(FromArgs, Debug)]
/// Focus the previous window
#[argh(subcommand, name = "focus-prev")]
struct FocusPrevArgs {}

#[derive(FromArgs, Debug)]
/// Toggle on the current space between BSP and Stack.
#[argh(subcommand, name = "toggle-space-layout")]
struct ToggleSpaceLayoutArgs {}

#[derive(FromArgs, Debug)]
/// Focus the next display, rotating through
#[argh(subcommand, name = "focus-next-display")]
struct FocusNextDisplayArgs {}

fn get_windows_for_space() -> Result<Vec<Window>> {
    // Execute the `yabai` command to get the JSON output for windows
    let output = Command::new("yabai")
        .args(&["-m", "query", "--windows", "--space"])
        .output()
        .expect("Failed to execute yabai command");

    // Ensure the command succeeded
    if !output.status.success() {
        panic!("Command returned an error: {:?}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut windows: Vec<Window> = serde_json::from_str(&stdout)?;

    windows.sort_by_key(|w| (w.display, w.frame.y as i64, w.frame.x as i64, w.id));

    Ok(windows)
}

fn get_display_config() -> Result<Vec<Display>> {
    // Execute the `yabai` command to get the JSON output for windows
    let output = Command::new("yabai")
        .args(&["-m", "query", "--displays"])
        .output()
        .expect("Failed to execute yabai command");

    // Ensure the command succeeded
    if !output.status.success() {
        panic!("Command returned an error: {:?}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(serde_json::from_str(&stdout)?)
}

fn get_space_config() -> Result<Space> {
    // Execute the `yabai` command to get the JSON output for windows
    let output = Command::new("yabai")
        .args(&["-m", "query", "--spaces", "--space"])
        .output()
        .expect("Failed to execute yabai command");

    // Ensure the command succeeded
    if !output.status.success() {
        panic!("Command returned an error: {:?}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(serde_json::from_str(&stdout)?)
}

fn focus_window_after(mut wins: Vec<Window>) -> Result<()> {
    wins.retain(|w| w.is_visible);
    if wins.is_empty() {
        return Ok(());
    }
    let idx = wins.iter().position(|w| w.has_focus).unwrap_or(0);
    let w = wins[(idx + 1) % wins.len()].id;
    Command::new("yabai")
        .args(&["-m", "window", "--focus", &format!("{w}")])
        .status()
        .expect("Failed to execute yabai command");
    Ok(())
}

fn focus_next_display() -> Result<()> {
    let displays = get_display_config()?;
    let idx = displays.iter().position(|d| d.has_focus).unwrap_or(0);
    let w = displays[(idx + 1) % displays.len()].id;
    Command::new("yabai")
        .args(&["-m", "display", "--focus", &format!("{w}")])
        .status()
        .expect("Failed to execute yabai command");
    Ok(())
}

fn focus_next_window() -> Result<()> {
    let wins = get_windows_for_space()?;
    focus_window_after(wins)
}

fn focus_prev_window() -> Result<()> {
    let mut wins = get_windows_for_space()?;
    wins.reverse();
    focus_window_after(wins)
}

fn toggle_space_layout() -> Result<()> {
    let new_layout = match get_space_config()?.space_type {
        SpaceType::Bsp => "stack",
        SpaceType::Stack => "bsp",
        SpaceType::Float => return Ok(()),
    };
    Command::new("yabai")
        .args(&["-m", "space", "--layout", new_layout])
        .status()
        .expect("Failed to execute yabai command");
    Ok(())
}

fn main() -> Result<()> {
    // Parse the command-line arguments
    let cli: Args = argh::from_env();

    match cli.cmd {
        SubCommand::FocusNext(_) => focus_next_window()?,
        SubCommand::FocusPrev(_) => focus_prev_window()?,
        SubCommand::ToggleSpaceLayout(_) => toggle_space_layout()?,
        SubCommand::FocusNextDisplay(_) => focus_next_display()?,
    }
    Ok(())
}
