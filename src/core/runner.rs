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

    let mut tail = Tail::new(settings.values.path_to_target_file.clone());

    let lines = Arc::new(Mutex::new(tail.read_lines(settings.values.last_lines_count)));

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
    extern fn callback(_: u32) {
        running!(false);
    };

    unsafe {
        signal::signal(SIGINT, callback);
        signal::signal(SIGQUIT, callback);
    }
}
