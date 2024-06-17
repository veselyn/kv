pub fn format<S>(input: S) -> anyhow::Result<String>
where
    S: Into<String>,
{
    let c_input = std::ffi::CString::new(input.into())?;

    let mut memstream = Memstream::open()?;
    log::trace!(memstream:?; "opened memstream");

    unsafe {
        let jv = jq_sys::jv_parse(c_input.as_ptr());

        log::trace!(memstream:?; "dumping jv to memstream");
        jq_sys::jv_dumpf(
            jv,
            memstream.file as *mut jq_sys::FILE,
            (jq_sys::jv_print_flags_JV_PRINT_PRETTY | jq_sys::jv_print_flags_JV_PRINT_SPACE2)
                as i32,
        );
        log::trace!(memstream:?; "dumped jv to memstream");
    };

    memstream.flush();
    log::trace!(memstream:?; "flushed memstream");

    let c_output = unsafe { std::ffi::CStr::from_ptr(memstream.buffer) };
    let output = c_output.to_str().map(|str| str.to_string());

    memstream.close()?;
    log::trace!(memstream:?; "closed memstream");

    Ok(output?)
}

#[derive(Debug)]
struct Memstream {
    buffer: *mut i8,
    size: usize,
    file: *mut libc::FILE,
    closed: bool,
}

impl Memstream {
    fn open() -> anyhow::Result<Self> {
        let mut memstream = Self {
            buffer: std::ptr::null_mut(),
            size: 0,
            file: std::ptr::null_mut(),
            closed: false,
        };

        let file = unsafe { libc::open_memstream(&mut memstream.buffer, &mut memstream.size) };
        anyhow::ensure!(!file.is_null());
        memstream.file = file;

        Ok(memstream)
    }

    fn flush(&self) {
        unsafe {
            libc::fflush(self.file);
        };
    }

    fn close(&mut self) -> anyhow::Result<()> {
        if self.closed {
            return Ok(());
        }

        let status = unsafe { libc::fclose(self.file) };
        unsafe { libc::free(self.buffer as *mut libc::c_void) }

        anyhow::ensure!(!status.is_positive());

        self.closed = true;

        Ok(())
    }
}

impl Drop for Memstream {
    fn drop(&mut self) {
        assert!(self.closed, "memstream not closed");
    }
}
