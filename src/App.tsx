import { useState, useEffect } from "preact/hooks";
import { DropZone } from "./components/DropZone";
import { QueueList } from "./components/QueueList";
import { ProgressBar } from "./components/ProgressBar";
import { ActionButtons } from "./components/ActionButtons";
import { listen } from "@tauri-apps/api/event";
import type { JobInfo, ProgressInfo } from "./types";

export function App() {
  const [jobs, setJobs] = useState<JobInfo[]>([]);
  const [currentProgress, setCurrentProgress] = useState<ProgressInfo | null>(null);

  useEffect(() => {
    // Listen for progress updates
    const progressUnlisten = listen<ProgressInfo>("processing-progress", (event) => {
      setCurrentProgress(event.payload);
      setJobs((prev) =>
        prev.map((job) =>
          job.status === "processing"
            ? { ...job, progress: event.payload }
            : job
        )
      );
    });

    // Listen for job completion
    const completeUnlisten = listen<{ jobId: string; outputPath: string }>(
      "job-complete",
      (event) => {
        setJobs((prev) =>
          prev.map((job) =>
            job.id === event.payload.jobId
              ? { ...job, status: "success", outputPath: event.payload.outputPath }
              : job
          )
        );
        setCurrentProgress(null);
      }
    );

    // Listen for job failure
    const failedUnlisten = listen<{ jobId: string; error: string }>(
      "job-failed",
      (event) => {
        setJobs((prev) =>
          prev.map((job) =>
            job.id === event.payload.jobId
              ? { ...job, status: "failed", error: event.payload.error }
              : job
          )
        );
        setCurrentProgress(null);
      }
    );

    return () => {
      progressUnlisten.then((unlisten) => unlisten());
      completeUnlisten.then((unlisten) => unlisten());
      failedUnlisten.then((unlisten) => unlisten());
    };
  }, []);

  const handleFilesAdded = (newJobs: JobInfo[]) => {
    setJobs((prev) => [...prev, ...newJobs]);
  };

  const handleCancel = async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("cancel_current");
  };

  const handleClearFinished = async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("clear_finished");
    setJobs((prev) => prev.filter((job) => job.status === "pending" || job.status === "processing"));
  };

  return (
    <div>
      <h1>Zip Image Converter</h1>
      <p className="subtitle">Convert images in zip files to JPEG format</p>

      <DropZone onFilesAdded={handleFilesAdded} />

      {currentProgress && (
        <ProgressBar progress={currentProgress} />
      )}

      <ActionButtons
        onCancel={handleCancel}
        onClearFinished={handleClearFinished}
        hasActiveJob={jobs.some((j) => j.status === "processing")}
        hasFinishedJobs={jobs.some((j) => j.status === "success" || j.status === "failed" || j.status === "cancelled")}
      />

      <QueueList jobs={jobs} />
    </div>
  );
}
