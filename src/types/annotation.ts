export type Point = {
  x: number;
  y: number;
};

export type Tool = "rectangle" | "arrow" | "draw";

export type Annotation =
  | {
      type: "rectangle";
      start: Point;
      end: Point;
      color: string;
      strokeWidth: number;
    }
  | {
      type: "arrow";
      start: Point;
      end: Point;
      color: string;
      strokeWidth: number;
    }
  | {
      type: "draw";
      points: Point[];
      color: string;
      strokeWidth: number;
    };

export const annotationTools: Array<{ label: string; value: Tool }> = [
  { label: "Rectangle", value: "rectangle" },
  { label: "Arrow", value: "arrow" },
  { label: "Draw", value: "draw" },
];
