use crate::{TaiResult, android::tools::gradlew, common::task::Task};

use super::Context;

pub struct BuildAndroidTest;

impl Task<Context> for BuildAndroidTest {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;

        let android_working_dir = project_meta.android_working_dir.to_owned();

        gradlew::assemble_android_test(&android_working_dir)?;
        gradlew::assemble_debug(&android_working_dir)?;

        let apks = android_working_dir.join("app").join("build").join("outputs").join("apk");
        let _test_apk = apks.join("androidTest").join("debug").join("app-debug-androidTest.apk");
        let _app_apk = apks.join("debug").join("app-debug.apk");

        Ok(context)
    }
}
