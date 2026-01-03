export interface JobInfo {
  id: string;
  inputPath: string;
  status: "pending" | "processing" | "success" | "failed" | "cancelled";
  progress?: ProgressInfo;
  outputPath?: string;
  error?: string;
}

export interface ProgressInfo {
  currentFile: number;
  totalFiles: number;
  currentFilename: string;
  phase: "scanning" | "converting" | "packaging";
}

export interface ProcessingStats {
  filesScanned: number;
  filesIncluded: number;
  filesConverted: number;
  filesSkipped: number;
}
