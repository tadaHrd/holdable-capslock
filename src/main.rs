use winapi::um::winuser::{keybd_event, GetKeyState, VK_CAPITAL, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP};
use std::time::{Instant, Duration};

fn main() {
    let mut start_time = Instant::now();
    
    let mut display_capslock_state = false;
    
    execute_arg_shit(std::env::args().collect::<Vec<String>>(), &mut display_capslock_state);
    
    loop {
        let elapsed = start_time.elapsed();
        
        let state = unsafe { GetKeyState(VK_CAPITAL) };
        let is_toggled_on = (state as u16 & 0x0001 as u16) != 0;
        let is_on = (state as u16 & 0x8000 as u16) != 0;
        
        if elapsed >= Duration::from_millis(300) {
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
    }
}

fn execute_arg_shit(args: Vec<String>, display_capslock_state: &mut bool) {
    for arg in args {
        match arg
            .replace("-", "")
            .replace("_", "")
            .as_str() {
                "displaytoggle" => {
                    *display_capslock_state = true;
                },
                _ => ()
            }
    }
}
