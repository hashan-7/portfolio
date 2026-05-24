interface ConfirmDialogProps {
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  onCancel: () => void;
}

function ConfirmDialog({
  open,
  title,
  message,
  confirmLabel = 'Delete',
  cancelLabel = 'Cancel',
  onConfirm,
  onCancel,
}: ConfirmDialogProps) {
  if (!open) {
    return null;
  }

  return (
    <div className="confirm-backdrop" role="presentation">
      <div className="confirm-dialog" role="dialog" aria-modal="true">
        <h3>{title}</h3>
        <p>{message}</p>

        <div className="confirm-actions">
          <button className="admin-secondary-button" type="button" onClick={onCancel}>
            {cancelLabel}
          </button>
          <button className="admin-danger-button" type="button" onClick={onConfirm}>
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}

export default ConfirmDialog;