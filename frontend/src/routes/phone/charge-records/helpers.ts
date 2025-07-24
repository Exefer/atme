import type { ChargeRecord } from "$lib/types/backend/bindings";
import { formatDuration, intervalToDuration } from "date-fns";

export const formatTimestamp = (timestamp: Date | string | null) =>
  timestamp ? new Date(timestamp).toLocaleString() : "-";

export const formatDurationFromMs = (ms: number) =>
  formatDuration(intervalToDuration({ start: 0, end: ms })); // -> 2 minutes 30 seconds

export const calculateDuration = (start: Date | string, end: Date | string | null) =>
  end
    ? formatDuration(
        intervalToDuration({ start: new Date(start).valueOf(), end: new Date(end).valueOf() })
      )
    : "-"; // -> 2 minutes 30 seconds

export const formatPercentage = (percentage: number | null, precision: number = 2) =>
  percentage ? `${percentage.toFixed(precision)}%` : "-";

export const calculateStats = (records: ChargeRecord[]) => {
  const completed = records.filter((s) => s.end_timestamp);
  const [totalDuration, totalCharge] = completed.reduce(
    ([d, c], s) => [
      d + (new Date(s.end_timestamp!).valueOf() - new Date(s.start_timestamp).valueOf()),
      c + s.end_percentage!
    ],
    [0, 0]
  );
  return {
    totalRecords: records.length,
    avgDuration: completed.length ? totalDuration / completed.length : 0,
    avgChargePercentage: completed.length ? totalCharge / completed.length : 0
  };
};
