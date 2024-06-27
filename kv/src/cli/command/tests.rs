use super::*;
use pretty_assertions::assert_eq;

#[test]
fn output_dumps_stdout() {
    let output = Output::default().stdout("stdout".to_owned());

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr);

    assert_eq!("stdout\n".as_bytes(), stdout);
    assert_eq!("".as_bytes(), stderr);
}

#[test]
fn output_dumps_stderr() {
    let output = Output::default().stderr("stderr".to_owned());

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr);

    assert_eq!("".as_bytes(), stdout);
    assert_eq!("stderr\n".as_bytes(), stderr);
}

#[test]
fn output_dumps_both() {
    let output = Output::default()
        .stdout("stdout".to_owned())
        .stderr("stderr".to_owned());

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr);

    assert_eq!("stdout\n".as_bytes(), stdout);
    assert_eq!("stderr\n".as_bytes(), stderr);
}

#[test]
fn error_dumps_message() {
    let error = Error::default().message("message".to_owned());

    let mut message = vec![];
    error.dump_to(&mut message);

    assert_eq!("Error: message\n".as_bytes(), message);
}
