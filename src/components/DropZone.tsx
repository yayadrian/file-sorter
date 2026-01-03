import { useState } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { JobInfo } from "../types";

interface DropZoneProps {
  onFilesAdded: (jobs: JobInfo[]) => void;
}

export function DropZone({ onFilesAdded }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);

  const handleFiles = async (paths: string[]) => {
    // Filter only .zip files
    const zipPaths = paths.filter((path) => path.toLowerCase().endsWith(".zip"));
    
    if (zipPaths.length === 0) {
      return;
    }

    try {
      const jobs = await invoke<JobInfo[]>("enqueue_zips", { paths: zipPaths });
      onFilesAdded(jobs);
    } catch (error) {
      console.error("Failed to enqueue files:", error);
      alert(`Error: ${error}`);
    }
  };

  const handleChooseFiles = async () => {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "Zip Files",
            extensions: ["zip"],
          },
        ],
      });

      if (selected && Array.isArray(selected)) {
        await handleFiles(selected);
      } else if (selected) {
        await handleFiles([selected]);
      }
    } catch (error) {
      console.error("Failed to open file dialog:", error);
    }
  };

  const handleDragOver = (e: DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  };

  const handleDragLeave = (e: DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  };

  const handleDrop = async (e: DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    if (!e.dataTransfer) return;

    const files = Array.from(e.dataTransfer.files);
    const paths = files.map((file) => (file as any).path).filter(Boolean);
    
    if (paths.length > 0) {
      await handleFiles(paths);
    }
  };

  return (
    <div
      class={`drop-zone ${isDragging ? "dragging" : ""}`}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      onClick={handleChooseFiles}
    >
      <div style={{ pointerEvents: "none" }}>
        <div style={{ fontSize: "3rem", marginBottom: "1rem" }}>📦</div>
        <h2>Drop Zip Files Here</h2>
        <p>
          or click to choose files
        </p>
        <p style={{ marginTop: "0.5rem", fontSize: "0.875rem", opacity: 0.7 }}>
          Only .zip files will be processed
        </p>
      </div>
    </div>
  );
}
