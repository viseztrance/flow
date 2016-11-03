/**
 * Flow - Realtime log analyzer
 * Copyright (C) 2016 Daniel Mircea
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use ext::signal::{self, SIGINT, SIGQUIT};
use utils::settings::Settings;
use core::tail::Tail;
use core::flow::Flow;

lazy_static! {
    pub static ref RUNNING: AtomicBool = AtomicBool::new(true);
}

pub fn execute(settings: Settings) {
    catch_signal();

    let mut tail = Tail::new(settings.path_to_target_file.clone());

    let lines = Arc::new(Mutex::new(tail.read_lines(settings.last_lines_count)));

    let reader_lines = lines.clone();
    let reader_thread = thread::spawn(move || {
        tail.watch(|data| reader_lines.lock().unwrap().extend(data));
    });

    let consumer_lines = lines.clone();
    let consumer_thread = thread::spawn(move || {
        let mut flow = Flow::new(settings);
        flow.render();
        flow.process(consumer_lines);
        flow.terminate();
    });

    let _ = reader_thread.join();
    let _ = consumer_thread.join();
}

fn catch_signal() {
    extern "C" fn callback(_: u32) {
        running!(false);
    };

    unsafe {
        signal::signal(SIGINT, callback);
        signal::signal(SIGQUIT, callback);
    }
}
