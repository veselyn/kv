#[cfg(test)]
mod tests;

use std::io::Read;

use anyhow::Context;

use crate::app::App;

impl App {
    pub fn json_get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        let value: String = self.db.query_row(
            "SELECT json(value) as value FROM keys WHERE id = :key AND type = 'json'",
            rusqlite::named_params! {
                ":key": key.into()
            },
            |row| row.get("value"),
        )?;

        let mut jq_value = String::with_capacity(value.len());
        let temp_file = tempfile::NamedTempFile::new()?;
        let temp_path = temp_file
            .path()
            .to_str()
            .context("Path is not valid unicode")?;

        unsafe {
            let jv = jq_sys::jv_parse(std::ffi::CString::new(value)?.as_ptr());

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
        temp_file.read_to_string(&mut jq_value)?;

        Ok(jq_value)
    }

    pub fn json_set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "INSERT OR REPLACE INTO keys (id, type, value) VALUES (:key, 'json', json(:value))",
            rusqlite::named_params! {
                ":key": key.into(),
                ":value": value.into()
            },
        )?;
        Ok(())
    }

    pub fn json_del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "DELETE FROM keys WHERE id = :key AND type = 'json'",
            rusqlite::named_params! {
                ":key": key.into()
            },
        )?;
        Ok(())
    }
}
