#![deny(unsafe_code)]
#![no_main] // this prog doesnt use main interface, we define entry point from cortex-m-rt crate's entry attribute
#![no_std] // dont include std crate

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

fn simple_iter(curr: i32) -> i32 {
    return (curr + 1) % 7
}

#[entry]
fn main() -> ! { // entry point must have fn() -> ! signature. This indicates function cant return = prog never terminates
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut period = 50_u16;
    let v_period = Volatile::new(&mut period);

    let mut curr: i32 = 0;
    let mut next: i32 = simple_iter(curr);
    let mut hiccup: bool = false;

    loop {
        // leds[0].on().ok();
        // delay.delay_ms(v_half_period.read());

        // leds[0].off().ok();
        // delay.delay_ms(v_half_period.read());
        // on logic
        leds[curr as usize].on().ok();
        leds[next as usize].on().ok();

        delay.delay_ms(v_period.read());

        // iteration logic
        if hiccup == false {
            // 2 leds were on
            leds[curr as usize].off().ok();
            curr = simple_iter(curr);
        }   
        else if hiccup == true {
            next = simple_iter(next);
        }

        hiccup = hiccup == false;
        
    }
}

