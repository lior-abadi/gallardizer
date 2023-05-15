use super::Detector;
use crate::utils::file_processor::FileNameWithContent;
use solang_parser::pt::SourceUnitPart;

pub struct PragmaVersionDetector;

impl Detector for PragmaVersionDetector {
    fn run_detector(&self, parsed_file: &FileNameWithContent) {
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::PragmaDirective(def, _opt, _opt_lit) => {
                    let pragma_version = _opt_lit.as_ref().map(|s| s.string.clone()).unwrap();
                    self.detector_logic(&pragma_version);
                }
                _ => (),
            }
        }
    }

    fn detector_logic(&self, content: &str) {
        println!("{content}");
        if !content.contains("^") && !content.contains(">") {
            println!("Replace fixed pragma version");
        }
    }
}
