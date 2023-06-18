use enigo::{Enigo, MouseButton::Left, MouseControllable};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tauri_hotkey::{Hotkey, HotkeyManager, Key, Modifier};

use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_enum, default_value_t=Mode::Oblit)]
    mode: Mode,
    #[arg(short, long, default_value_t = 2000)]
    speed: usize,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Oblit,
    Disenchant,
    Debug,
    LeftClick,
    RightClick,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.mode);

    let mut enigo = Enigo::new();
    let mut kb_manager = HotkeyManager::new();
    let keys = vec![Key::SEMICOLON];
    let modifiers = vec![];
    let r_key = Hotkey { keys, modifiers };

    let run = Arc::new(Mutex::new(false));
    let run_ = Arc::clone(&run);
    kb_manager
        .register(r_key, move || {
            let mut run = run_.lock().unwrap();
            *run = !*run;
            println!(
                "Current state is {}",
                if *run { "running" } else { "not running" }
            );
        })
        .expect("cannot use keybard manager register enevents");

    loop {
        if !*run.lock().unwrap() {
            sleep(Duration::from_millis(args.speed as u64));
            continue;
        }
        match args.mode {
            // If debug -> print mouse pos
            Mode::Debug => {
                sleep(Duration::from_millis(args.speed as u64));
                let pos = enigo.mouse_location();
                println!("{:?}", pos);
            }

            Mode::Disenchant => {
                enigo.mouse_move_to(3862, 978);
                enigo.mouse_click(Left);
                sleep(Duration::from_millis(2300));
                enigo.mouse_move_to(2960, 1125);
                enigo.mouse_click(Left);
                sleep(Duration::from_millis(100));
                enigo.mouse_move_to(4703, 750);
                enigo.mouse_click(Left);
                sleep(Duration::from_millis(2000));
            }
            Mode::Oblit => {
                // Tailoring craft button click
                enigo.mouse_move_to(3862, 978);
                enigo.mouse_click(Left);
                sleep(Duration::from_millis(2200));

                // Drag item to oblit slot
                enigo.mouse_move_to(4703, 750);
                enigo.mouse_down(Left);
                sleep(Duration::from_millis(200));
                enigo.mouse_move_to(2800, 335);
                sleep(Duration::from_millis(200));
                enigo.mouse_up(Left);

                // Clic oblit
                sleep(Duration::from_millis(200));
                enigo.mouse_move_to(2800, 430);
                enigo.mouse_click(Left);
                sleep(Duration::from_millis(2100));
            }
            Mode::RightClick => {
                sleep(Duration::from_millis(args.speed as u64));
                enigo.mouse_click(enigo::MouseButton::Right);
            }
            Mode::LeftClick => {
                sleep(Duration::from_millis(args.speed as u64));
                enigo.mouse_click(enigo::MouseButton::Left);
            }
            _ => {}
        }
    }
}
