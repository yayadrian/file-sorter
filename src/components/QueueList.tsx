import { QueueItem } from "./QueueItem";
import type { JobInfo } from "../types";

interface QueueListProps {
  jobs: JobInfo[];
}

export function QueueList({ jobs }: QueueListProps) {
  if (jobs.length === 0) {
    return null;
  }

  return (
    <div className="queue-container">
      <h2>Queue</h2>
      <div className="queue-list">
        {jobs.map((job) => (
          <QueueItem key={job.id} job={job} />
        ))}
      </div>
    </div>
  );
}
