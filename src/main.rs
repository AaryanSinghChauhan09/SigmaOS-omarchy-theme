//! SigmaOS-omarchy-theme — Wallust-style theme CLI.
//!
//! Usage:
//!   omarchy-theme list
//!   omarchy-theme current
//!   omarchy-theme apply <name>
//!   omarchy-theme cycle
//!   omarchy-theme css <name>      (print CSS vars)
//!   omarchy-theme hyprland <name> (print hyprland col.* lines)
//!   omarchy-theme colors <name>   (print 16 ANSI color indices)

use omarchy_theme::{OmarchyTheme, ThemeError};
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: omarchy-theme <list|current|apply|cycle|css|hyprland|colors> [name]");
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() { usage(); }
    let mut themes = OmarchyTheme::new();
    let cmd = args[0].as_str();
    let name = args.get(1).map(|s| s.as_str());

    let result: Result<(), String> = match cmd {
        "list" => {
            for t in themes.themes() {
                println!("{:<20} {:<5} {} colors", t.name, t.variant.label(), t.palette.len());
            }
            Ok(())
        }
        "current" => { println!("{}", themes.status_line()); Ok(()) }
        "apply" => {
            let n = name.unwrap_or_else(|| usage());
            themes.apply(n).map(|_| println!("applied {n}")).map_err(|e| e.to_string())
        }
        "cycle" => {
            match themes.cycle() {
                Some(n) => { println!("cycled -> {n}"); Ok(()) }
                None => Err("no themes".into()),
            }
        }
        "css" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => { print!("{}", t.css_vars()); Ok(()) }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        "hyprland" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => { print!("{}", t.hyprland_colors()); Ok(()) }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        "colors" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => {
                    let idx = t.terminal_colors();
                    println!("{}", idx.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
                    Ok(())
                }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        _ => usage(),
    };

    if let Err(e) = result {
        eprintln!("omarchy-theme: {e}");
        exit(1);
    }
}
