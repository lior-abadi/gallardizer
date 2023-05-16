mod engine;
mod utils;

use engine::gallardizer_engine as gallardizer;
use engine::parser as solc_parser;
use utils::file_processor;

fn main() {
    run_detectors();

    // solc_parser::parse_target();
}

fn run_detectors() {
    let solidity_files: Vec<file_processor::FileNameWithContent> =
        file_processor::get_all_solidity_files("./");

    gallardizer::gallardize(solidity_files)
}
