use super::detectors;
use super::parser;
use crate::utils::file_processor::FileNameWithContent;

pub fn gallardize(targets: Vec<FileNameWithContent>) {
    let parsed_files: Vec<FileNameWithContent> = parser::parse_targets(targets);

    detectors::run_all_detectors(parsed_files)
}
