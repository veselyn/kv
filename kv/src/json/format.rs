use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("input contains nul character: {0}")]
    NulInput(#[from] std::ffi::NulError),
    #[error("output is not valid utf-8: {0}")]
    InvalidOutput(#[from] std::str::Utf8Error),
    #[error("opening memstream file")]
    OpenMemstreamFile,
    #[error("closing memstream file: {0}")]
    CloseMemstreamFile(libc::c_int),
}

pub fn format<S>(input: S) -> Result<String, Error>
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

    let c_output = unsafe { std::ffi::CStr::from_ptr(*memstream.buffer) };
    let output = c_output.to_str().map(|str| str.to_string());

    memstream.close()?;
    log::trace!(memstream:?; "closed memstream");

    Ok(output?)
}

#[derive(Debug)]
struct Memstream {
    buffer: *mut *mut libc::c_char,
    size: *mut libc::size_t,
    file: *mut libc::FILE,
    closed: bool,
}

impl Memstream {
    fn open() -> Result<Self, Error> {
        let mut memstream = Self {
            buffer: Box::into_raw(Box::new(std::ptr::null_mut())),
            size: Box::into_raw(Box::new(0)),
            file: std::ptr::null_mut(),
            closed: false,
        };

        let file = unsafe { libc::open_memstream(memstream.buffer, memstream.size) };
        if file.is_null() {
            return Err(Error::OpenMemstreamFile);
        }

        memstream.file = file;

        Ok(memstream)
    }

    fn flush(&self) {
        unsafe {
            libc::fflush(self.file);
        };
    }

    fn close(&mut self) -> Result<(), Error> {
        if self.closed {
            return Ok(());
        }

        let status = unsafe { libc::fclose(self.file) };
        unsafe { libc::free(*self.buffer as *mut libc::c_void) }

        if status.is_positive() {
            return Err(Error::CloseMemstreamFile(status));
        }

        unsafe {
            drop(Box::from_raw(self.buffer));
            drop(Box::from_raw(self.size));
        };
        self.buffer = std::ptr::null_mut();
        self.size = std::ptr::null_mut();
        self.file = std::ptr::null_mut();

        self.closed = true;

        Ok(())
    }
}

impl Drop for Memstream {
    fn drop(&mut self) {
        self.close().expect("closing memstream");
    }
}
