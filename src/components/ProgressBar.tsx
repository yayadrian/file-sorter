import type { ProgressInfo } from "../types";

interface ProgressBarProps {
  progress: ProgressInfo;
}

export function ProgressBar({ progress }: ProgressBarProps) {
  const percentage = progress.totalFiles > 0
    ? (progress.currentFile / progress.totalFiles) * 100
    : 0;

  const phaseText = {
    scanning: "Scanning",
    converting: "Converting",
    packaging: "Packaging",
    extracting: "Extracting",
  }[progress.phase] || progress.phase;

  return (
    <div className="progress-container">
      <div style={{ marginBottom: "0.75rem", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <span style={{ fontWeight: "600", fontSize: "0.875rem" }}>
          {phaseText} {progress.currentFile}/{progress.totalFiles}
        </span>
        <span style={{ fontSize: "0.875rem", color: "var(--text-secondary)", fontVariantNumeric: "tabular-nums" }}>
          {percentage.toFixed(0)}%
        </span>
      </div>
      
      <div
        style={{
          width: "100%",
          height: "8px",
          backgroundColor: "rgba(255, 255, 255, 0.1)",
          borderRadius: "999px",
          overflow: "hidden",
        }}
      >
        <div
          style={{
            width: `${percentage}%`,
            height: "100%",
            backgroundColor: "var(--primary-color)",
            borderRadius: "999px",
            transition: "width 0.3s ease",
            boxShadow: "0 0 10px rgba(59, 130, 246, 0.5)"
          }}
        />
      </div>
      
      {progress.currentFilename && (
        <div style={{ marginTop: "0.75rem", fontSize: "0.75rem", color: "var(--text-secondary)", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
          {progress.currentFilename}
        </div>
      )}
    </div>
  );
}
