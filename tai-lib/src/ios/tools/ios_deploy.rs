use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const IOS_DEPLOY: &str = "ios-deploy";

#[derive(Debug)]
pub struct IosDeployLaunch<'a, 'e> {
    device: String,
    bundle: PathBuf,
    args: Option<&'a [String]>,
    envs: Option<&'e [(String, String)]>,
    non_interactive: bool,
    debug: bool,
    no_wifi: bool,
    verbose: bool,
    app_deltas: Option<PathBuf>,
}

impl<'a, 'e> IosDeployLaunch<'a, 'e> {
    pub fn new<B: AsRef<Path>>(device: &str, bundle: B) -> Self {
        Self {
            device: device.to_owned(),
            bundle: bundle.as_ref().to_owned(),
            args: None,
            envs: None,
            non_interactive: false,
            debug: false,
            no_wifi: false,
            verbose: false,
            app_deltas: None,
        }
    }

    pub fn args(&mut self, args: &'a [String]) -> &mut Self {
        self.args = Some(args);
        self
    }
    pub fn envs(&mut self, envs: &'e [(String, String)]) -> &mut Self {
        self.envs = Some(envs);
        self
    }

    pub fn non_interactive(&mut self) -> &mut Self {
        self.non_interactive = true;
        self
    }

    pub fn debug(&mut self) -> &mut Self {
        self.debug = true;
        self
    }

    pub fn no_wifi(&mut self) -> &mut Self {
        self.no_wifi = true;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn app_deltas<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.app_deltas = Some(path.as_ref().to_owned());
        self
    }

    pub fn execute(&mut self) -> TaiResult<()> {
        let mut cmd = Command::new(IOS_DEPLOY);
        if !self.verbose {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        cmd.arg("--id").arg(&self.device);

        self.non_interactive
            .then(|| ())
            .map(|_| cmd.arg("--noninteractive"));
        self.debug.then(|| ()).map(|_| cmd.arg("--debug"));
        self.no_wifi.then(|| ()).map(|_| cmd.arg("--no-wifi"));

        if let Some(args) = self.args {
            cmd.args(&["--args", &args.join(" ")]);
        };

        if let Some(envs) = self.envs {
            let envs_as_string = envs
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join(" ");
            cmd.args(&["--envs", &envs_as_string]);
        };

        self.app_deltas
            .as_ref()
            .map(|path| cmd.arg("--app_deltas").arg(path));

        cmd.arg("--bundle").arg(&self.bundle);

        cmd.status()?.expect_success("failed to run ios_deploy")
    }
}
