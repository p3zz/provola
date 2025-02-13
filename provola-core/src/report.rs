use serde::{Deserialize, Serialize};

pub type Timestamp = chrono::DateTime<chrono::Utc>;
pub type Duration = std::time::Duration;
pub type Count = usize;
pub type Name = String;
pub type Status = String;
pub type Hostname = String;
pub type Id = String;
pub type Package = String;
pub type ClassName = String;
pub type FailureType = String;
pub type Message = String;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Report {
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    /// The total number of rule violations
    pub failures: Option<Count>,
    /// The label of the scan
    pub name: Option<Name>,
    /// The total number of rules that were applied
    pub tests: Option<Count>,
    pub testsuites: Vec<TestSuite>,
    /// The time that was required to process all the rules
    pub time: Option<Duration>,
    pub timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
    // TODO properties
    // TODO system-err
    // TODO system-out
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    pub failures: Option<Count>,
    pub hostname: Option<Hostname>,
    pub id: Option<Id>,
    /// The label of the provider
    pub name: Name,
    pub package: Option<Package>,
    pub skipped: Option<Count>,
    pub testcases: Vec<TestCase>,
    /// The number of rules in the provider that were applied
    pub tests: Count,
    /// The time that was required to process the rules in the provider
    pub time: Option<Duration>,
    pub timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestCase {
    // TODO error
    // TODO assertions
    // TODO skipped
    // TODO system-err
    // TODO system-out
    pub classname: Option<ClassName>,
    /// The label of the rule
    pub name: Name,
    pub status: Option<Status>,
    /// The time that was required to process all the applications of this rule
    pub time: Option<Duration>,
    pub failures: Vec<Failure>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Failure {
    pub ttype: FailureType,
    pub message: Message,
}
