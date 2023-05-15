pub mod pragma_version;

use crate::utils::file_processor::FileNameWithContent;
use solang_parser::pt::SourceUnit;

pub trait Detector {
    fn run_detector(&self, parsed_file: &FileNameWithContent);
    fn detector_logic(&self, content: &str);
}

pub fn run_all_detectors(parsed_files: Vec<FileNameWithContent>) {
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(pragma_version::PragmaVersionDetector),
        /* Add more detector modules */
    ];

    for detector in detectors {
        for file in &parsed_files {
            detector.run_detector(&file);
        }
    }
}
