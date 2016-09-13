extern crate flow;

use std::env;

use flow::settings::SettingsBuilder;

fn main() {
    let args = env::args().collect();

    let settings = SettingsBuilder::new(args).construct();
    flow::runner::execute(settings);
}
