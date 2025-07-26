export type ChargeRecord = {
  id: number;
  start_timestamp: string;
  end_timestamp: string | null;
  start_percentage: number;
  end_percentage: number | null;
};
