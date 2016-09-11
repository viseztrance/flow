use std::sync::{Arc, Mutex};
use std::thread;

use settings::Settings;
use flow::tail::Tail;
use flow::flow::Flow;

pub fn execute(settings: Settings) {
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
