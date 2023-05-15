use super::detectors;
use super::parser;

use solang_parser::pt::SourceUnit;

pub fn gallardize(targets: Vec<&str>) {
    let parsed_files: Vec<SourceUnit> = parser::parse_targets(targets);
    detectors::run_all_detectors(parsed_files)
}
