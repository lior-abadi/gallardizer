use super::detectors;
use super::parser;
use crate::utils::file_processor::FileNameWithContent;
use solang_parser::pt::SourceUnit;

pub fn gallardize(targets: Vec<FileNameWithContent>) {
    let parsed_files: Vec<FileNameWithContent> = parser::parse_targets(targets);
    // println!("{:?}", parsed_files);
    detectors::run_all_detectors(parsed_files)
}
