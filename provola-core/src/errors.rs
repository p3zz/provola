use crate::test_runners::TestRunnerFeature;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no test result available")]
    NoResult,
    #[error("executable not available")]
    NoExecutable,
    #[error("cannot build: {0}")]
    BuildFailed(String),
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
    #[error("language not supported: {0}")]
    LangNotSupported(String),
    #[error("cannot execute")]
    ExecError(#[from] subprocess::PopenError),
    #[error(transparent)]
    InvalidInputData(std::io::Error),
    #[error(transparent)]
    InvalidOutputData(std::io::Error),
    #[error("nothing to do")]
    NothingToDo,
    #[error("not implemented")]
    NotImplemented,
    #[error("cannot watch file: {0}")]
    CannotWatch(String),
    #[error("test runner not supported: {0}")]
    TestRunnerNotSupported(String),
    #[error("reporter error")]
    ReporterError(#[from] crate::reporter::Error),
    #[error("report unavailable")]
    ReportUnavailable,
    #[error("feature not available: {0}")]
    TestRunnerFeatureNotAvailable(TestRunnerFeature),
    #[error("cannot parse report: {0}")]
    ReportParseError(Box<dyn std::error::Error>),
}
