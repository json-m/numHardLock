#![allow(non_snake_case)]
use std::thread::sleep;
use std::time::Duration;
use windows::{
    Win32::UI::Input::KeyboardAndMouse::{
        INPUT,
        INPUT_0,
        KEYBDINPUT,
        VIRTUAL_KEY,
        SendInput,
        GetKeyState,
        INPUT_KEYBOARD,
        KEYEVENTF_KEYUP,
    },
};

// idk why i need to do this
const NUMBER_LOCK:u16 = 144;

// get state of number lock, return bool
fn get_numlock_state() -> bool {
    unsafe {
        if GetKeyState(NUMBER_LOCK as i32) == 1 {
            true
        } else {
            false
        }
    }
}

unsafe fn enable_numlock() {
    // define a key down and key up array of inputs
    let mut keypress = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(NUMBER_LOCK),
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                }
            }
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(NUMBER_LOCK),
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            }
        }
    ];

    // use SendInput to turn on NUMBER_LOCK
    if SendInput(&mut keypress, std::mem::size_of::<INPUT>() as _) == 1 {
        println!("failed to send numLock keypress")
    }

}

fn main() {
    unsafe {
        println!("numHardLock starting");

        loop {
            if !get_numlock_state() {
                //println!("numLock off, turning on");
                enable_numlock()
            }
            sleep(Duration::from_millis(250))
        }

    }
}
