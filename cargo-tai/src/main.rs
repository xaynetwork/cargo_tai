use anyhow::Error;

use structopt::StructOpt;
use tai_lib::common::command::run_command;

mod opts;

use opts::Options;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

fn main() -> Result<(), Error> {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    let opt = Options::from_args();
    let requested_opt: tai_lib::common::opts::Options = opt.into();

    #[cfg(not(target_os = "macos"))]
    {
        // workaround because this is not possible:
        //
        // [target.'cfg(not(target_os="macos"))'.dependencies]
        // tai-lib = { path = "../tai-lib" }

        // [target.'cfg(target_os="macos")'.dependencies]
        // tai-lib = { path = "../tai-lib", features = ["ios"] }
        //
        // it would be possible if we switch to resolver = "2" but
        // it might be to early

        use cfg_expr::targets::Os;
        if let Some(Os::ios) = &requested_opt.compiler.target.os {
            panic!("cannot compile any iOS targets on a non Apple host system")
        }
    }

    run_command(requested_opt)
}
