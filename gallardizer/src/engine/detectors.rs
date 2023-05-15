pub mod pragma_version;

use solang_parser::pt::SourceUnit;

pub trait Detector {
    fn run_detector(&self, parsed_file: &SourceUnit);
}

pub fn run_all_detectors(parsed_files: Vec<SourceUnit>) {
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(pragma_version::PragmaVersionDetector), /* Add more detector modules */
    ];

    for detector in detectors {
        for file in &parsed_files {
            detector.run_detector(&file);
        }
    }
}
