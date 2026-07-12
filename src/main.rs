mod cpu;
mod mmu;
mod registers;

use cpu::CPU;
use raylib::prelude::*;

struct State {
    cpu: CPU,
    debug: bool,
    cont: bool,
    next: bool,
    close: bool,
    restart: bool,
}

impl State {
    fn new() -> Self {
        Self {
            cpu: CPU::new(),
            debug: false,
            cont: false,
            next: false,
            close: false,
            restart: false,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("RustBoy").build();

    let mut state = State::new();

    while !rl.window_should_close() {
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        manage_keys(&rl, &mut state);

        if state.close {
            break;
        }
        if state.restart {
            state = State::new();
        }
        if state.next || state.cont {
            state.cpu.step();
        }
        state.next = false;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        draw_debug(d, screen_width, screen_height, &state);
    }
}

fn manage_keys(rl: &RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(KeyboardKey::KEY_D) {
        state.debug = !state.debug;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_C) {
        state.cont = !state.cont;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_N) {
        state.next = true;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_R) {
        state.restart = true;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_Q) {
        state.close = true;
    }
}

fn draw_debug(mut d: RaylibDrawHandle, screen_width: i32, screen_height: i32, state: &State) {
    if !state.debug {
        return;
    }

    let cpu = &state.cpu;

    d.draw_text(
        format!("A: {:#x}", cpu.registers.a).as_str(),
        10,
        10,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("B: {:#x}", cpu.registers.b).as_str(),
        10,
        30,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("C: {:#x}", cpu.registers.c).as_str(),
        10,
        50,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("D: {:#x}", cpu.registers.d).as_str(),
        10,
        70,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("E: {:#x}", cpu.registers.e).as_str(),
        10,
        90,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("F: {:#x} {:#b}", cpu.registers.f, cpu.registers.f).as_str(),
        10,
        110,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("H: {:#x}", cpu.registers.h).as_str(),
        10,
        130,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("L: {:#x}", cpu.registers.l).as_str(),
        10,
        150,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("AF: {:#x}", cpu.registers.af()).as_str(),
        10,
        170,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("BC: {:#x}", cpu.registers.bc()).as_str(),
        10,
        190,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("DE: {:#x}", cpu.registers.de()).as_str(),
        10,
        210,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("HL: {:#x} {:#b}", cpu.registers.hl(), cpu.registers.hl()).as_str(),
        10,
        230,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("SP: {:#x}", cpu.registers.sp).as_str(),
        10,
        250,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("PC: {:#x} {:#b}", cpu.registers.pc, cpu.registers.pc).as_str(),
        10,
        270,
        20,
        Color::WHITE,
    );
    d.draw_text(
        format!("OP: {:#x}", cpu.mmu.read_byte(cpu.registers.pc)).as_str(),
        10,
        290,
        20,
        Color::YELLOW,
    );

    d.draw_fps(screen_width - 100, 10);
}
