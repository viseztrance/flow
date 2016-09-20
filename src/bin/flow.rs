extern crate flow;

use std::env;

use flow::utils::settings::SettingsBuilder;

fn main() {
    let args = env::args().collect();

    let settings = SettingsBuilder::new(args).construct();
    flow::core::runner::execute(settings);
}
