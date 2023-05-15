use solang_parser::{
    parse,
    pt::{ContractPart, SourceUnit, SourceUnitPart},
};

pub fn parse_targets(targets: Vec<&str>) -> Vec<SourceUnit> {
    let mut parsed_trees: Vec<SourceUnit> = vec![];
    for target in targets {
        let literal_target: String = format!(r#"{}"#, target);
        let (tree, comments) = parse(literal_target.as_str(), 0).unwrap();
        parsed_trees.push(tree);
    }

    return parsed_trees;
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
