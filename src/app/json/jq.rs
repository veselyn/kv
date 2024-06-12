use std::io::Read;

use anyhow::Context;

pub fn format<S>(input: S) -> anyhow::Result<String>
where
    S: Into<String>,
{
    let input: String = input.into();

    let mut formatted = String::with_capacity(input.len());

    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_path = temp_file
        .path()
        .to_str()
        .context("Path is not valid unicode")?;

    unsafe {
        let jv = jq_sys::jv_parse(std::ffi::CString::new(input)?.as_ptr());

        let c_temp_file = libc::fopen(
            std::ffi::CString::new(temp_path)?.as_ptr(),
            std::ffi::CString::new("w")?.as_ptr(),
        );
        anyhow::ensure!(!c_temp_file.is_null());

        jq_sys::jv_dumpf(
            jv,
            c_temp_file as *mut jq_sys::FILE,
            (jq_sys::jv_print_flags_JV_PRINT_PRETTY | jq_sys::jv_print_flags_JV_PRINT_SPACE2)
                .try_into()?,
        );

        let status = libc::fclose(c_temp_file);
        anyhow::ensure!(status == 0);
    };

    let mut temp_file = std::fs::File::open(temp_path)?;
    temp_file.read_to_string(&mut formatted)?;

    Ok(formatted)
}
