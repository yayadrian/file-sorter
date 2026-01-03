interface ActionButtonsProps {
  onCancel: () => void;
  onClearFinished: () => void;
  hasActiveJob: boolean;
  hasFinishedJobs: boolean;
}

export function ActionButtons({
  onCancel,
  onClearFinished,
  hasActiveJob,
  hasFinishedJobs,
}: ActionButtonsProps) {
  return (
    <div style={{ display: "flex", gap: "1rem", justifyContent: "center", margin: "2rem 0" }}>
      <button
        onClick={onCancel}
        disabled={!hasActiveJob}
        className="btn btn-danger"
      >
        Cancel Current
      </button>
      
      <button
        onClick={onClearFinished}
        disabled={!hasFinishedJobs}
        className="btn btn-secondary"
      >
        Clear Finished
      </button>
    </div>
  );
}
