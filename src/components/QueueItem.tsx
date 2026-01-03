import { invoke } from "@tauri-apps/api/core";
import type { JobInfo } from "../types";

interface QueueItemProps {
  job: JobInfo;
}

export function QueueItem({ job }: QueueItemProps) {
  const handleOpenFolder = async () => {
    if (job.outputPath) {
      try {
        await invoke("open_in_folder", { path: job.outputPath });
      } catch (error) {
        console.error("Failed to open folder:", error);
      }
    }
  };

  const statusLabel = {
    pending: "Pending",
    processing: "Processing",
    success: "Success",
    failed: "Failed",
    cancelled: "Cancelled",
  }[job.status];

  const fileName = job.inputPath.split(/[\\/]/).pop() || job.inputPath;

  return (
    <div className="queue-item">
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", marginBottom: "0.5rem" }}>
        <div style={{ flex: 1, marginRight: "1rem" }}>
          <div style={{ fontWeight: "600", marginBottom: "0.25rem", wordBreak: "break-all" }}>
            {fileName}
          </div>
          <div style={{ fontSize: "0.75rem", color: "var(--text-secondary)" }}>
            {job.inputPath}
          </div>
        </div>
        
        <span className={`status-badge status-${job.status}`}>
          {statusLabel}
        </span>
      </div>

      {job.status === "processing" && job.progress && (
        <div style={{ fontSize: "0.875rem", color: "var(--text-secondary)", marginTop: "0.75rem" }}>
          {job.progress.phase === "scanning" && "Scanning files..."}
          {job.progress.phase === "converting" && `Converting ${job.progress.currentFile}/${job.progress.totalFiles}`}
          {job.progress.phase === "packaging" && "Creating output zip..."}
        </div>
      )}

      {job.status === "success" && job.outputPath && (
        <div style={{ marginTop: "1rem", display: "flex", alignItems: "center", justifyContent: "space-between", gap: "1rem" }}>
          <div style={{ fontSize: "0.875rem", color: "var(--text-secondary)", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
            Saved to: {job.outputPath}
          </div>
          <button
            onClick={handleOpenFolder}
            className="btn btn-secondary"
            style={{ padding: "0.25rem 0.75rem", fontSize: "0.75rem" }}
          >
            Show in Folder
          </button>
        </div>
      )}

      {job.status === "failed" && job.error && (
        <div style={{ marginTop: "0.5rem", fontSize: "0.875rem", color: "var(--error-color)" }}>
          Error: {job.error}
        </div>
      )}
    </div>
  );
}
