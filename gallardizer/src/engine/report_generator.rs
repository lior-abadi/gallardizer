#[derive(Debug, Clone)]
pub struct GasParams {
    pub amount: i64,
}

#[derive(Debug, Clone)]
pub enum Severities {
    GAS(GasParams),
    NC,
    L,
    M,
    H,
}

#[derive(Debug, Clone)]
pub struct IssueAppearance {
    pub file_path: String,
    pub line: u16,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct IssueMetadata {
    pub title: String,
    pub content: String,
    pub severity: Severities,
}

// An issue is composed by several appearances
#[derive(Clone, Debug)]
pub struct Issue {
    pub issue_appearances: Vec<IssueAppearance>,
    pub metadata: IssueMetadata,
}
