use winapi::um::winuser::{keybd_event, GetKeyState, VK_CAPITAL, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP};
use std::{time::{Instant, Duration}, thread};

fn main() {
    let mut start_time = Instant::now();

    let mut display_capslock_state = true;
    let mut time = 300;
    let mut wait_time = 20;

    execute_arg_shit(std::env::args().collect::<Vec<String>>(), &mut display_capslock_state, &mut time, &mut wait_time);

    loop {
        let elapsed = start_time.elapsed();

        let state = unsafe { GetKeyState(VK_CAPITAL) };
        let is_toggled_on = (state as u16 & 0x0001 as u16) != 0;
        let is_on = (state as u16 & 0x8000 as u16) != 0;

        if elapsed >= Duration::from_millis(time as u64) {
            if !is_on & is_toggled_on {
                unsafe {
                    keybd_event(VK_CAPITAL as u8, 0x3a, KEYEVENTF_EXTENDEDKEY, 0);
                    keybd_event(VK_CAPITAL as u8, 0x3a, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
                }
                start_time = Instant::now();
            }
        }

        if !is_on {
            start_time = Instant::now();
        }
        if display_capslock_state {
            match (is_toggled_on, is_on) {
                (true, true)   => print!("toggled on and held     \r"),
                (true, false)  => print!("toggled on and released \r"),
                (false, true)  => print!("toggled off and held    \r"),
                (false, false) => print!("toggled off and released\r"),
            }
        }
        thread::sleep(Duration::from_millis(wait_time));
    }
}

fn execute_arg_shit(args: Vec<String>, display_capslock_state: &mut bool, time: &mut i32, wait_time: &mut u64) {
    let mut is_time_arg = false;
    let mut is_wait_arg = false;

    for arg in args {
        if is_time_arg {
            is_time_arg = false;
            *time = arg.parse::<i32>().unwrap();
        }
        if is_wait_arg {
            is_wait_arg = false;
            *wait_time = arg.parse::<u64>().unwrap();
        }
        match arg
            .replace("-", "")
            .replace("_", "")
            .as_str() {
                "displayoff" | "nodisplay" =>  *display_capslock_state = false,
                "notoggle" | "onlyhold" => *time = 0,
                "time" | "t" => is_time_arg = true,
                "waittime" | "wait" => is_wait_arg = true,
                _ => (),
            }
    }
}
