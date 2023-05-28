use crate::engine::detectors::{get_appearance_metadata_with_comments, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use ispell::{SpellChecker, SpellLauncher};
use solang_parser::pt::{Comment, Loc};

pub struct Typos {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for Typos {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let mut typo_checker = SpellLauncher::new()
            .aspell()
            .dictionary("en_GB")
            .launch()
            .unwrap();

        for comment in &parsed_file.comments {
            // As all type of comments have the same structure, we capture the content

            match comment {
                Comment::Line(loc, content)
                | Comment::Block(loc, content)
                | Comment::DocLine(loc, content)
                | Comment::DocBlock(loc, content) => {
                    if content.contains("SPDX") {
                        continue;
                    }

                    let lines_of_comment: Vec<&str> = comment.value().lines().collect();

                    // Scan each line of comment
                    let mut misspelled_words: Vec<String> = vec![];

                    for line_of_comment in lines_of_comment.clone() {
                        let trimmed_line = line_of_comment.trim();
                        if trimmed_line == "*"
                            || trimmed_line == "/*"
                            || trimmed_line == "/**"
                            || trimmed_line == "*/"
                        {
                            continue;
                        }

                        let line_errors_result = typo_checker.check(trimmed_line);

                        if let Ok(line_errors) = line_errors_result {
                            for error in line_errors {
                                if error.misspelled.contains("ERC")
                                    || error.misspelled.contains("arg")
                                    || error.misspelled.contains("struct")
                                    || error.misspelled.contains("enum")
                                    || error.misspelled.contains("dev")
                                    || error.misspelled.contains("param")
                                    || error.misspelled.contains("inheritdoc")
                                    || error.misspelled.contains("immutable")
                                    || error.misspelled.contains("immutable")
                                {
                                    continue;
                                }

                                misspelled_words.push(error.misspelled)
                            }
                        }
                    }
                    // If no misspelled words were found, continue
                    if misspelled_words.len() == 0 {
                        continue;
                    }

                    // Evaluate matches
                    let mut max_match_count = 0;
                    let mut matching_line_index = 0;

                    for (index, line) in lines_of_comment.iter().enumerate() {
                        let match_count = count_matches(line, misspelled_words.clone());

                        if match_count > max_match_count {
                            max_match_count = match_count;
                            matching_line_index = index;
                        }
                    }

                    if let Loc::File(_, initial_position, _final_position) = loc {
                        let matching_line_content = lines_of_comment[matching_line_index];
                        let adjusted_start_offset = initial_position
                            + lines_of_comment
                                .iter()
                                .take(matching_line_index)
                                .map(|line| line.len() + 1)
                                .sum::<usize>();
                        let adjusted_end_offset =
                            adjusted_start_offset + matching_line_content.len();

                        let newLoc = Loc::File(0, adjusted_start_offset, adjusted_end_offset);
                        let mut typo_errors: String = String::new();
                        for misspelled_word in misspelled_words {
                            typo_errors += &(misspelled_word + " ")
                        }

                        typo_errors = typo_errors.trim().to_owned();

                        self.detected_issues
                            .push(get_appearance_metadata_with_comments(
                                &newLoc,
                                parsed_file,
                                &typo_errors,
                            ));
                    }
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "Typos".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Typos"}.to_string(),
            content: indoc! {""}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn count_matches(line: &str, misspelled_words: Vec<String>) -> usize {
    let mut amount_of_matches: usize = 0;
    for word in misspelled_words {
        if line.contains(&word) {
            amount_of_matches += 1;
        }
    }
    return amount_of_matches;
}
