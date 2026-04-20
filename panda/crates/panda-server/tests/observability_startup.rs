use std::process::Command;

#[test]
fn cli_help_lists_config_and_env_section() {
    let output = Command::new(env!("CARGO_BIN_EXE_panda"))
        .arg("--help")
        .output()
        .expect("run panda --help");
    assert!(output.status.success(), "help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("panda.yaml") && stdout.contains("CONFIG"),
        "expected config hint in help: {stdout}"
    );
    assert!(
        stdout.contains("PANDA_REDIS_URL") || stdout.contains("OTEL_EXPORTER_OTLP_ENDPOINT"),
        "expected env section in help: {stdout}"
    );
    assert!(
        stdout.contains("--ui") && stdout.contains("--print-live-trace-schema"),
        "expected console flags in help: {stdout}"
    );
}

#[test]
fn print_live_trace_schema_emits_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_panda"))
        .arg("--print-live-trace-schema")
        .output()
        .expect("run panda --print-live-trace-schema");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("live_trace") && stdout.contains("\"kind\""),
        "expected schema-like JSON: {stdout}"
    );
}

#[test]
fn startup_with_otlp_env_fails_only_on_missing_config() {
    let missing = tempfile::tempdir()
        .expect("tempdir")
        .path()
        .join("does-not-exist.yaml");

    let output = Command::new(env!("CARGO_BIN_EXE_panda"))
        .arg(missing)
        .env(
            "OTEL_EXPORTER_OTLP_ENDPOINT",
            "http://127.0.0.1:4318/v1/traces",
        )
        .env("PANDA_OTEL_SERVICE_NAME", "panda-test")
        .output()
        .expect("run panda binary");

    assert!(
        !output.status.success(),
        "expected non-zero status for missing config"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to load config"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "unexpected panic output: {stderr}"
    );
}
