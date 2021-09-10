use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

use anyhow::anyhow;

use crate::{
    android::compiler::{bench_command, benches_command, test_command, tests_command},
    command::Command,
    compiler::{compile_benches, compile_tests},
    options::{self, GeneralOptions},
    task::Task,
    TaiResult,
};

use super::Context;

pub struct BuildBuildUnit;

impl Task for BuildBuildUnit {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let requested: Options = context.requested.clone().try_into()?;
        let sdk = context.android_sdk()?;
        let general_opt = &context.requested.general;

        let cmd = match general_opt.command {
            Command::Bench => bench_command(sdk, &requested)?,
            Command::Test => test_command(sdk, &requested)?,
            Command::Benches => benches_command(sdk, &requested)?,
            Command::Tests => tests_command(sdk, &requested)?,
        };

        let build_units = match general_opt.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &general_opt.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &general_opt.compiler)?,
        };
        context.build_units = Some(build_units);

        Ok(context)
    }
}

pub struct Options {
    pub general: GeneralOptions,

    pub android_api_lvl: u8,
    pub android_ndk: PathBuf,
}

impl TryFrom<options::Options> for Options {
    type Error = anyhow::Error;

    fn try_from(opt: options::Options) -> Result<Self, Self::Error> {
        Ok(Self {
            general: opt.general,
            android_api_lvl: opt
                .platform
                .android_api_lvl
                .ok_or_else(|| anyhow!("the option android_api_lvl is missing"))?,
            android_ndk: opt
                .platform
                .android_ndk
                .ok_or_else(|| anyhow!("the option android_ndk is missing"))?,
        })
    }
}
