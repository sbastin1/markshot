export type ScreenshotResult = {
  data_url: string;
};

export type HistoryItem = {
  path: string;
  image_url: string;
  created_at: number;
};

export type SaveResult = {
  path: string;
  copied: boolean;
  warning: string | null;
};
