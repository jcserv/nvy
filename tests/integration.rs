use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;
use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;

struct TestEnv {
    temp_dir: TempDir,
}

impl TestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        Self { temp_dir }
    }

    fn create_env_file(&self, name: &str, contents: &str) -> std::io::Result<()> {
        let mut file = File::create(self.temp_dir.path().join(name))?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()
    }

    fn create_config(&self, contents: &str) -> std::io::Result<()> {
        let mut file = File::create(self.temp_dir.path().join("nvy.yaml"))?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()
    }

    fn assert_config_exists(&self) -> bool {
        self.temp_dir.path().join("nvy.yaml").exists()
    }

    fn get_config_contents(&self) -> String {
        fs::read_to_string(self.temp_dir.path().join("nvy.yaml")).unwrap()
    }
}

#[test]
fn test_init_creates_config_in_empty_directory() {
    let env = TestEnv::new();
    
    env.create_env_file(".env", "APP_ENV=default").unwrap();
    env.create_env_file(".env.local", "APP_ENV=local").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=production").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .assert()
        .success();

    assert!(env.assert_config_exists());
    
    let contents = env.get_config_contents();
    let expected_config = r#"target: .env.nvy
profiles:
  default:
  - path: .env
  local:
  - path: .env.local
  prod:
  - path: .env.prod
"#;
    assert_eq!(contents, expected_config);
}

#[test]
fn test_init_prompts_for_reinit_accepts() {
    let env = TestEnv::new();
    
    env.create_config(r#"target: .env.nvy
profiles:
  default:
    - path: .env"#).unwrap();
    
    env.create_env_file(".env", "APP_ENV=default").unwrap();
    env.create_env_file(".env.test", "APP_ENV=test").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .write_stdin("y\n")
        .assert()
        .success();

    let contents = env.get_config_contents();
    let expected_config = r#"target: .env.nvy
profiles:
  default:
  - path: .env
  test:
  - path: .env.test
"#;
    assert_eq!(contents, expected_config);
}

#[test]
fn test_init_prompts_for_reinit_declines() {
    let env = TestEnv::new();
    
    let initial_config = r#"profiles:
  default:
    - path: .env"#;
    env.create_config(initial_config).unwrap();
    
    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .write_stdin("n\n")
        .assert()
        .success();

    assert_eq!(env.get_config_contents().trim(), initial_config);
}

#[test]
fn test_use_happy_path() {
    let env = TestEnv::new();
    
    env.create_env_file(".env", "APP_ENV=default\nAPI_KEY=123").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=production\nAPI_KEY=456").unwrap();
    
    env.create_config(r#"target: sh
profiles:
  default:
    - path: .env
  prod:
    - path: .env.prod"#).unwrap();

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("prod")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(output.contains("export APP_ENV='production'"));
    assert!(output.contains("export API_KEY='456'"));
    assert!(output.contains("export NV_CURRENT_PROFILE='prod'"));
}

#[test]
fn test_use_unsets_previous_profile() {
    let env = TestEnv::new();
    
    env.create_env_file(".env", "APP_ENV=default\nAPI_KEY=123").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=production").unwrap();
    
    env.create_config(r#"target: sh
profiles:
  default:
    - path: .env
  prod:
    - path: .env.prod"#).unwrap();

    std::env::set_var("NV_CURRENT_PROFILE", "default");
    std::env::set_var("APP_ENV", "default");
    std::env::set_var("API_KEY", "123");

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("prod")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(output.contains("unset API_KEY"));
    assert!(output.contains("export APP_ENV='production'"));
}

#[test]
fn test_use_fails_without_init() {
    let env = TestEnv::new();
    
    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("prod")
        .current_dir(&env.temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("nvy.yaml does not exist"));
}

#[test]
fn test_use_fails_with_nonexistent_profile() {
    let env = TestEnv::new();
    
    env.create_config(r#"target: sh
profiles:
  default:
    - path: .env"#).unwrap();
    env.create_env_file(".env", "APP_ENV=default").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("nonexistent")
        .current_dir(&env.temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Profile nonexistent does not exist"));
}

#[test]
fn test_use_fails_with_missing_env_file() {
    let env = TestEnv::new();
    
    env.create_config(r#"target: sh
profiles:
  prod:
    - path: .env.prod"#).unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("prod")
        .current_dir(&env.temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_use_default_when_no_args() {
    let env = TestEnv::new();
    
    env.create_env_file(".env", "DEFAULT=1").unwrap();
    env.create_config(r#"target: sh
profiles:
  default:
    - path: .env"#).unwrap();

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .current_dir(&env.temp_dir)
        .assert()
        .success();

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(output.contains("export DEFAULT='1'"));
    assert!(output.contains("export NV_CURRENT_PROFILE='default'"));
}

#[test]
fn test_init_preserves_custom_target() {
    let env = TestEnv::new();
    
    // Create initial config with custom target
    env.create_config(r#"target: .env.local
profiles:
  default:
    - path: .env"#).unwrap();
    
    env.create_env_file(".env", "APP_ENV=default").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=production").unwrap();
    env.create_env_file(".env.local", "").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .write_stdin("y\n")
        .assert()
        .success();

    let contents = env.get_config_contents();
    let expected_config = r#"target: .env.local
profiles:
  default:
  - path: .env
  prod:
  - path: .env.prod
"#;
    assert_eq!(contents, expected_config);
}

#[test]
fn test_use_multiple_profiles() {
    let env = TestEnv::new();
    
    env.create_env_file(".env.base", "SHARED=base\nBASE_ONLY=value").unwrap();
    env.create_env_file(".env.override", "SHARED=override\nOVERRIDE_ONLY=value").unwrap();
    
    env.create_config(r#"target: sh
profiles:
  base:
    - path: .env.base
  override:
    - path: .env.override"#).unwrap();

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("base")
        .arg("override")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let actual = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    let expected = r#"# base
export BASE_ONLY='value'

# override
export SHARED='override'
export OVERRIDE_ONLY='value'

export NV_CURRENT_PROFILE='base,override'
"#;
    
    assert_eq!(actual, expected);
}

#[test]
fn test_use_multiple_profiles_with_custom_target() {
    let env = TestEnv::new();
    
    env.create_env_file(".env.base", "SHARED=base\nBASE_ONLY=value").unwrap();
    env.create_env_file(".env.override", "SHARED=override\nOVERRIDE_ONLY=value").unwrap();
    env.create_env_file(".env.target", "").unwrap();
    
    env.create_config(r#"target: .env.target
profiles:
  base:
    - path: .env.base
  override:
    - path: .env.override"#).unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("base")
        .arg("override")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let actual = fs::read_to_string(env.temp_dir.path().join(".env.target")).unwrap();

    let expected = r#"# base
BASE_ONLY=value

# override
SHARED=override
OVERRIDE_ONLY=value
"#;
    
    assert_eq!(actual, expected);
}

#[test]
fn test_init_ignores_target_env_file() {
    let env = TestEnv::new();
    
    env.create_config(r#"target: .env.local
profiles:
  default:
    - path: .env"#).unwrap();
    
    env.create_env_file(".env", "APP_ENV=default").unwrap();
    env.create_env_file(".env.local", "APP_ENV=local").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=prod").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .write_stdin("y\n")
        .assert()
        .success();

    let contents = env.get_config_contents();
    assert!(!contents.contains("local:"));
    assert!(contents.contains("prod:"));
}

#[test]
fn test_use_profile_with_invalid_env_vars() {
    let env = TestEnv::new();
    
    env.create_env_file(".env.test", r#"VALID_VAR=123
INVALID-VAR=456
ANOTHER_VALID=789
INVALID@VAR=abc"#).unwrap();
    
    env.create_config(r#"target: sh
profiles:
  test:
    - path: .env.test"#).unwrap();

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("test")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(output.contains("export VALID_VAR='123'"));
    assert!(output.contains("export ANOTHER_VALID='789'"));
    assert!(!output.contains("INVALID-VAR"));
    assert!(!output.contains("INVALID@VAR"));
}

#[test]
fn test_use_with_empty_profile() {
    let env = TestEnv::new();
    
    env.create_env_file(".env.empty", "").unwrap();
    
    env.create_config(r#"target: sh
profiles:
  empty:
    - path: .env.empty"#).unwrap();

    let assert = AssertCommand::cargo_bin("nvy").unwrap()
        .arg("use")
        .arg("empty")
        .current_dir(&env.temp_dir)
        .assert()
        .success();
    
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(output.trim(), "export NV_CURRENT_PROFILE='empty'");
}

#[test]
fn test_init_ignores_example_env() {
    let env = TestEnv::new();
    
    env.create_env_file(".env", "APP_ENV=default").unwrap();
    env.create_env_file(".env.prod", "APP_ENV=prod").unwrap();
    env.create_env_file(".env.example", "APP_ENV=example\nDB_URL=example").unwrap();

    AssertCommand::cargo_bin("nvy").unwrap()
        .arg("init")
        .current_dir(&env.temp_dir)
        .assert()
        .success();

    let contents = env.get_config_contents();
    let expected_config = r#"target: .env.nvy
profiles:
  default:
  - path: .env
  prod:
  - path: .env.prod
"#;
    assert_eq!(contents, expected_config);
    
    assert!(!contents.contains("example"));
    assert!(!contents.contains(".env.example"));
}