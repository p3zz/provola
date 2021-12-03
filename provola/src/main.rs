use clap::Parser;
use provola_core::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "provola", about = "provola, the quick tester")]
struct Opt {
    /// Activate debug mode
    #[clap(long)]
    debug: bool,
    /// Watch files or directories for changes
    #[clap(short, long, parse(from_os_str))]
    watch: Option<PathBuf>,
    /// Input file
    #[clap(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
    /// Expected output
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Language
    #[clap(short, long)]
    lang: Option<Language>,
    /// Source code
    #[clap(short, long)]
    source: Option<PathBuf>,
}

impl From<&Opt> for Actions {
    fn from(opt: &Opt) -> Self {
        let mut actions = Vec::new();

        let lang = opt
            .lang
            .or_else(|| opt.source.as_ref().and_then(|x| Language::from_source(x)));

        if let (Some(lang), Some(source)) = (lang, &opt.source) {
            let source = Source::new(source.clone());
            actions.push(Action::Build(lang, source));
        }

        if let (Some(input), Some(output)) = (&opt.input, &opt.output) {
            let input = TestDataIn::new(input.clone());
            let output = TestDataOut::new(output.clone());
            actions.push(Action::TestInputOutput(input, output));
        }

        Actions(actions)
    }
}

fn watch(opt: &Opt, watch_files: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let (tx, rx) = channel();

    let debounce_time = Duration::from_secs(1);
    let mut watcher = watcher(tx, debounce_time).unwrap();

    watcher
        .watch(&watch_files, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(e) => {
                log::trace!("{:?}", e);
                run_once(opt).ok(); // TODO Print error and continue
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}

fn run_once(opt: &Opt) -> Result<(), Box<dyn std::error::Error>> {
    let actions = Actions::from(opt);
    let result = actions.run()?;
    let reporter = SimpleReporter::new();

    reporter.report(result);

    Ok(())
}

fn run(opt: &Opt) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(watch_files) = &opt.watch {
        run_once(opt).ok(); // TODO Print error and continue
        watch(opt, watch_files)
    } else {
        run_once(opt)
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::parse();

    if let Err(e) = run(&opt) {
        log::error!("{}", e);
    }
}
