use super::*;
use crate::env::stdio::{MemoryStderr, MemoryStdout};
use async_std::sync::{Arc, Mutex};
use pretty_assertions::assert_eq;

#[async_std::test]
async fn output_dumps_stdout() {
    let stdout = Arc::new(Mutex::new(MemoryStdout::new(false)));
    let stderr = Arc::new(Mutex::new(MemoryStderr::new(false)));

    let output = Output::from(
        Env::builder()
            .stdout(stdout.clone())
            .stderr(stderr.clone())
            .build(),
    )
    .stdout("stdout".to_owned());

    output.dump().await;

    assert_eq!(
        Ok("stdout\n".to_owned()),
        String::from_utf8(stdout.lock().await.buf.clone())
    );
    assert_eq!(
        Ok("".to_owned()),
        String::from_utf8(stderr.lock().await.buf.clone())
    );
}

#[async_std::test]
async fn output_dumps_stderr() {
    let stdout = Arc::new(Mutex::new(MemoryStdout::new(false)));
    let stderr = Arc::new(Mutex::new(MemoryStderr::new(false)));

    let output = Output::from(
        Env::builder()
            .stdout(stdout.clone())
            .stderr(stderr.clone())
            .build(),
    )
    .stderr("stderr".to_owned());

    output.dump().await;

    assert_eq!(
        Ok("".to_owned()),
        String::from_utf8(stdout.lock().await.buf.clone())
    );
    assert_eq!(
        Ok("stderr\n".to_owned()),
        String::from_utf8(stderr.lock().await.buf.clone())
    );
}

#[async_std::test]
async fn output_dumps_both() {
    let stdout = Arc::new(Mutex::new(MemoryStdout::new(false)));
    let stderr = Arc::new(Mutex::new(MemoryStderr::new(false)));

    let output = Output::from(
        Env::builder()
            .stdout(stdout.clone())
            .stderr(stderr.clone())
            .build(),
    )
    .stdout("stdout".to_owned())
    .stderr("stderr".to_owned());

    output.dump().await;

    assert_eq!(
        Ok("stdout\n".to_owned()),
        String::from_utf8(stdout.lock().await.buf.clone())
    );
    assert_eq!(
        Ok("stderr\n".to_owned()),
        String::from_utf8(stderr.lock().await.buf.clone())
    );
}

#[async_std::test]
async fn error_dumps_message() {
    let stdout = Arc::new(Mutex::new(MemoryStdout::new(false)));
    let stderr = Arc::new(Mutex::new(MemoryStderr::new(false)));

    let error = Error::from(
        Env::builder()
            .stdout(stdout.clone())
            .stderr(stderr.clone())
            .build(),
    )
    .message("message".to_owned());

    error.dump().await;

    assert_eq!(
        Ok("".to_owned()),
        String::from_utf8(stdout.lock().await.buf.clone()),
    );
    assert_eq!(
        Ok("Error: message\n".to_owned()),
        String::from_utf8(stderr.lock().await.buf.clone()),
    );
}

#[async_std::test]
async fn error_status_defaults_to_failure() {
    assert_eq!(1, Error::from(Env::new()).status);
}

#[async_std::test]
async fn error_status_can_be_set() {
    assert_eq!(2, Error::from(Env::new()).status(2).status);
}
