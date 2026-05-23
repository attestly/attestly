//! End-to-end integration test for the CLI binary.
//!
//! Runs the full pipeline: init → append → publish → export → verify (PASS)
//! → tamper → export → verify (FAIL). Mirrors `examples/demo.sh`, but lives
//! in CI so any regression in the tamper-detection path fails the build.

use std::path::PathBuf;
use std::process::Command;

fn attestly_binary() -> PathBuf {
    // Cargo provides CARGO_BIN_EXE_<bin-name> for integration tests.
    PathBuf::from(env!("CARGO_BIN_EXE_attestly"))
}

fn run(cmd: &mut Command) -> (i32, String, String) {
    let out = cmd.output().expect("spawn attestly");
    let code = out.status.code().unwrap_or(-1);
    (
        code,
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn e2e_clean_pass_then_tamper_fail() {
    let dir = tempfile::tempdir().expect("tempdir");
    let dir_path = dir.path();
    let bin = attestly_binary();
    let db = dir_path.join("attestly.db");
    let keys_dir = dir_path.join("keys");
    let checkpoint = dir_path.join("checkpoint.json");
    let bundle = dir_path.join("regulator-3.zip");
    let bundle_tampered = dir_path.join("regulator-3-tampered.zip");

    // 1. init
    let (code, _, stderr) = run(Command::new(&bin)
        .arg("init")
        .arg("--db")
        .arg(&db)
        .arg("--system-did")
        .arg("did:web:test.example/ai-system/credit")
        .arg("--org-did")
        .arg("did:web:test.example")
        .arg("--key-id")
        .arg("ops-test")
        .arg("--keys-dir")
        .arg(&keys_dir));
    assert_eq!(code, 0, "init failed: {stderr}");

    // 2. append 5 decisions (deterministic content so we know which is 'denied')
    for i in 1..=5 {
        let decision = if i % 3 == 0 { "denied" } else { "approved" };
        let data = format!(r#"{{"decision":"{decision}","applicant":"a{i}"}}"#);
        let (code, _, stderr) = run(Command::new(&bin)
            .arg("append")
            .arg("--db")
            .arg(&db)
            .arg("--keys-dir")
            .arg(&keys_dir)
            .arg("--data")
            .arg(&data));
        assert_eq!(code, 0, "append #{i} failed: {stderr}");
    }

    // 3. publish checkpoint
    let (code, _, stderr) = run(Command::new(&bin)
        .arg("publish-checkpoint")
        .arg("--db")
        .arg(&db)
        .arg("--keys-dir")
        .arg(&keys_dir)
        .arg("--out")
        .arg(&checkpoint));
    assert_eq!(code, 0, "publish failed: {stderr}");

    // 4. export evidence for seq=3 (which is 'denied')
    let (code, _, stderr) = run(Command::new(&bin)
        .arg("export")
        .arg("--db")
        .arg(&db)
        .arg("--keys-dir")
        .arg(&keys_dir)
        .arg("--decision-id")
        .arg("3")
        .arg("--checkpoint")
        .arg(&checkpoint)
        .arg("--out")
        .arg(&bundle));
    assert_eq!(code, 0, "export failed: {stderr}");

    // 5. verify clean — should PASS
    let (code, stdout, _) = run(Command::new(&bin).arg("verify").arg("--bundle").arg(&bundle));
    assert_eq!(code, 0, "clean verify should pass; stdout={stdout}");
    assert!(stdout.contains("[OK]"), "expected [OK] line; got: {stdout}");

    // 6. tamper seq=3 from 'denied' to 'approved'
    let (code, _, stderr) = run(Command::new(&bin)
        .arg("demo-tamper")
        .arg("--db")
        .arg(&db)
        .arg("--seq")
        .arg("3")
        .arg("--new-decision")
        .arg("approved"));
    assert_eq!(code, 0, "tamper failed: {stderr}");

    // 7. operator self-audit should now catch the tamper (canonical hash != stored)
    let (code, stdout, stderr) = run(Command::new(&bin)
        .arg("audit")
        .arg("--db")
        .arg(&db)
        .arg("--keys-dir")
        .arg(&keys_dir));
    assert_ne!(code, 0, "audit should exit non-zero after tamper; stdout={stdout}");
    assert!(stderr.contains("tamper") || stdout.contains("mismatched"), "stdout={stdout} stderr={stderr}");

    // 8. re-export and verify — should FAIL
    let (code, _, stderr) = run(Command::new(&bin)
        .arg("export")
        .arg("--db")
        .arg(&db)
        .arg("--keys-dir")
        .arg(&keys_dir)
        .arg("--decision-id")
        .arg("3")
        .arg("--checkpoint")
        .arg(&checkpoint)
        .arg("--out")
        .arg(&bundle_tampered));
    assert_eq!(code, 0, "re-export failed: {stderr}");

    let (code, _, stderr) = run(Command::new(&bin)
        .arg("verify")
        .arg("--bundle")
        .arg(&bundle_tampered));
    assert_eq!(code, 1, "tampered verify should exit 1; stderr={stderr}");
    assert!(stderr.contains("TAMPERED"), "expected TAMPERED in stderr; got: {stderr}");
}

#[test]
fn verify_malformed_bundle_exits_2() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bundle = dir.path().join("not-a-zip.zip");
    std::fs::write(&bundle, b"this is definitely not a zip file").unwrap();

    let bin = attestly_binary();
    let (code, _, stderr) = run(Command::new(&bin).arg("verify").arg("--bundle").arg(&bundle));
    assert_eq!(code, 2, "malformed verify should exit 2; stderr={stderr}");
    assert!(stderr.contains("malformed"), "stderr={stderr}");
}
