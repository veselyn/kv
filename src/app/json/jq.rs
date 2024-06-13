pub fn format<S>(input: S) -> anyhow::Result<String>
where
    S: Into<String>,
{
    let input: String = input.into();

    let formatted = unsafe {
        let jv = jq_sys::jv_parse(std::ffi::CString::new(input)?.as_ptr());

        let mut buffer: *mut libc::c_char = std::ptr::null_mut();
        let mut size: libc::size_t = 0;

        let memstream = libc::open_memstream(
            std::ptr::from_mut(&mut buffer),
            std::ptr::from_mut(&mut size),
        );
        anyhow::ensure!(!memstream.is_null());

        jq_sys::jv_dumpf(
            jv,
            memstream as *mut jq_sys::FILE,
            (jq_sys::jv_print_flags_JV_PRINT_PRETTY | jq_sys::jv_print_flags_JV_PRINT_SPACE2)
                .try_into()?,
        );

        let status = libc::fclose(memstream);
        anyhow::ensure!(status == 0);

        let formatted = std::ffi::CStr::from_ptr(buffer).to_str()?.to_string();

        libc::free(buffer as *mut libc::c_void);

        formatted
    };

    Ok(formatted)
}
