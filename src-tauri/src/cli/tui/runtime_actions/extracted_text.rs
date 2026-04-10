use std::path::PathBuf;

use crate::cli::i18n::texts;
use crate::error::AppError;

use super::super::app::ToastKind;
use super::RuntimeActionContext;

pub(super) fn copy(ctx: &mut RuntimeActionContext<'_>, content: String) -> Result<(), AppError> {
    match ctx
        .terminal
        .with_terminal_restored(|| crate::cli::osc52::copy_to_stdout(&content))
    {
        Ok(()) => ctx.app.push_toast(
            texts::tui_toast_clipboard_copy_requested(),
            ToastKind::Success,
        ),
        Err(err) => ctx.app.push_toast(
            texts::tui_toast_clipboard_copy_failed(&err.to_string()),
            ToastKind::Warning,
        ),
    }

    Ok(())
}

pub(super) fn open_external(
    ctx: &mut RuntimeActionContext<'_>,
    content: String,
) -> Result<(), AppError> {
    ctx.terminal.with_terminal_restored(|| {
        let _ = crate::cli::editor::open_external_editor(&content)?;
        Ok(())
    })
}

pub(super) fn save(
    ctx: &mut RuntimeActionContext<'_>,
    path: String,
    content: String,
) -> Result<(), AppError> {
    let target = PathBuf::from(path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|err| AppError::io(parent, err))?;
    }
    std::fs::write(&target, content.as_bytes()).map_err(|err| AppError::io(&target, err))?;
    ctx.app.push_toast(
        texts::tui_toast_exported_to(&target.display().to_string()),
        ToastKind::Success,
    );
    Ok(())
}
