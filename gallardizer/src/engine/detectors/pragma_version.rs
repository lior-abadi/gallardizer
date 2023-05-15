use super::Detector;
pub struct PragmaVersionDetector;
use solang_parser::pt::{SourceUnit, SourceUnitPart};

impl Detector for PragmaVersionDetector {
    fn run_detector(&self, parsed_file: &SourceUnit) {
        for part in &parsed_file.0 {
            match part {
                SourceUnitPart::PragmaDirective(def, _opt, _opt_lit) => {
                    let pragma_version = _opt_lit.as_ref().map(|s| s.string.clone()).unwrap();
                    println!("\n{:?}", pragma_version);

                    if !pragma_version.contains("^") && !pragma_version.contains(">") {
                        print!("Replace fixed pragma version");
                    }
                }
                _ => (),
            }
        }
    }
}
