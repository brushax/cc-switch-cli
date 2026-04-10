use std::io::{self, Write};

use base64::prelude::*;

use crate::error::AppError;

pub fn write_copy_sequence(mut writer: impl Write, content: &str) -> Result<(), AppError> {
    let encoded = BASE64_STANDARD.encode(content.as_bytes());
    write!(writer, "\x1b]52;c;{encoded}\x07").map_err(|source| AppError::IoContext {
        context: "write OSC52 clipboard sequence".to_string(),
        source,
    })?;
    writer.flush().map_err(|source| AppError::IoContext {
        context: "flush OSC52 clipboard sequence".to_string(),
        source,
    })
}

pub fn copy_to_stdout(content: &str) -> Result<(), AppError> {
    write_copy_sequence(io::stdout(), content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("boom"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_copy_sequence_uses_osc52_format() {
        let mut out = Vec::new();
        write_copy_sequence(&mut out, "hello\nworld").expect("write osc52 sequence");

        assert_eq!(
            String::from_utf8(out).expect("utf8"),
            "\u{1b}]52;c;aGVsbG8Kd29ybGQ=\u{7}"
        );
    }

    #[test]
    fn write_copy_sequence_wraps_io_failures_with_context() {
        let err = write_copy_sequence(FailingWriter, "hello").expect_err("expected io error");

        assert!(
            matches!(err, AppError::IoContext { ref context, .. } if context.contains("OSC52")),
            "unexpected error: {err:?}"
        );
    }
}
