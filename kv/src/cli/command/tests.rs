use super::*;
use io::Cursor;
use pretty_assertions::assert_eq;

#[test]
fn output_dumps_stdout() {
    let mut output = Output::default().stdout(Cursor::new("stdout"));

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr).unwrap();

    assert_eq!(Ok("stdout\n".to_owned()), String::from_utf8(stdout));
    assert_eq!(Ok("".to_owned()), String::from_utf8(stderr));
}

#[test]
fn output_dumps_stderr() {
    let mut output = Output::default().stderr(Cursor::new("stderr"));

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr).unwrap();

    assert_eq!(Ok("".to_owned()), String::from_utf8(stdout));
    assert_eq!(Ok("stderr\n".to_owned()), String::from_utf8(stderr));
}

#[test]
fn output_dumps_both() {
    let mut output = Output::default()
        .stdout(Cursor::new("stdout"))
        .stderr(Cursor::new("stderr"));

    let mut stdout = vec![];
    let mut stderr = vec![];
    output.dump_to(&mut stdout, &mut stderr).unwrap();

    assert_eq!(Ok("stdout\n".to_owned()), String::from_utf8(stdout));
    assert_eq!(Ok("stderr\n".to_owned()), String::from_utf8(stderr));
}

#[test]
fn error_dumps_message() {
    let error = Error::default().message("message".to_owned());

    let mut message = vec![];
    error.dump_to(&mut message).unwrap();

    assert_eq!(
        Ok("Error: message\n".to_owned()),
        String::from_utf8(message)
    );
}

#[test]
fn error_status_defaults_to_failure() {
    assert_eq!(1, Error::default().status);
}

#[test]
fn error_status_can_be_set() {
    assert_eq!(2, Error::default().status(2).status);
}
