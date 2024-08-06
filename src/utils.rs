use std::process::Command;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub fn exec(cmd: &str, args: Vec<&str>) -> String {
    let c = Command::new(cmd).args(args).output().unwrap().stdout;
    let res = String::from_utf8_lossy(&c);

    res.to_string()
}

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}
