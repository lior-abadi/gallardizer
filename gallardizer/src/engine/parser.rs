use crate::utils::file_processor::FileNameWithContent;
use solang_parser::parse;

pub fn parse_targets(targets: Vec<FileNameWithContent>) -> Vec<FileNameWithContent> {
    let mut parsed_files: Vec<FileNameWithContent> = Vec::new();
    for mut target in targets {
        let literal_target: String = format!(r#"{}"#, target.file_content);
        let (tree, comments) = parse(literal_target.as_str(), 0).unwrap();

        target.parsed_ast_tree = tree;
        parsed_files.push(target);
    }

    return parsed_files;
}

// for part in &tree.0 {
//     match part {
//         SourceUnitPart::ContractDefinition(def) => {
//             println!("found contract {:?}", def.name);
//             for part in &def.parts {
//                 match part {
//                     ContractPart::VariableDefinition(def) => {
//                         println!("variable {:?}", def.name);
//                     }
//                     ContractPart::FunctionDefinition(def) => {
//                         println!("function {:?}", def.name);
//                     }
//                     _ => (),
//                 }
//             }
//         }
//         SourceUnitPart::PragmaDirective(def, _opt, _opt_lit) => {
//             println!(
//                 "found pragma {:?} /// {:?} /// {:?} /// {:?}",
//                 def,
//                 _opt,
//                 _opt_lit,
//                 _opt_lit.as_ref().map(|s| s.string.clone()).unwrap()
//             );
//         }
//         _ => (),
//     }
// }
