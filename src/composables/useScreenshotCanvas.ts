import { nextTick, ref, type Ref } from "vue";
import type { Annotation, Point, Tool } from "../types/annotation";

export function useScreenshotCanvas(options: {
  selectedTool: Ref<Tool>;
  selectedColor: Ref<string>;
  strokeWidth: Ref<number>;
}) {
  const canvasElement = ref<HTMLCanvasElement | null>(null);
  const annotations = ref<Annotation[]>([]);

  let screenshotImage: HTMLImageElement | null = null;
  let activeAnnotation: Annotation | null = null;

  async function loadScreenshot(dataUrl: string) {
    await nextTick();

    const canvas = canvasElement.value;

    if (!canvas) {
      return;
    }

    const image = new Image();
    image.src = dataUrl;

    await new Promise<void>((resolve, reject) => {
      image.onload = () => resolve();
      image.onerror = () =>
        reject(new Error("Failed to load screenshot into editor"));
    });

    screenshotImage = image;
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    annotations.value = [];
    redrawCanvas();
  }

  function startAnnotation(event: PointerEvent) {
    const position = canvasPosition(event);

    if (!position) {
      return;
    }

    canvasElement.value?.setPointerCapture(event.pointerId);

    if (options.selectedTool.value === "draw") {
      activeAnnotation = {
        type: "draw",
        points: [position],
        color: options.selectedColor.value,
        strokeWidth: options.strokeWidth.value,
      };
    } else {
      activeAnnotation = {
        type: options.selectedTool.value,
        start: position,
        end: position,
        color: options.selectedColor.value,
        strokeWidth: options.strokeWidth.value,
      };
    }
  }

  function updateAnnotation(event: PointerEvent) {
    if (!activeAnnotation) {
      return;
    }

    const position = canvasPosition(event);

    if (!position) {
      return;
    }

    if (activeAnnotation.type === "draw") {
      activeAnnotation.points.push(position);
    } else {
      activeAnnotation.end = position;
    }

    redrawCanvas(activeAnnotation);
  }

  function finishAnnotation(event: PointerEvent) {
    if (!activeAnnotation) {
      return;
    }

    const canvas = canvasElement.value;

    if (canvas?.hasPointerCapture(event.pointerId)) {
      canvas.releasePointerCapture(event.pointerId);
    }

    if (isMeaningfulAnnotation(activeAnnotation)) {
      annotations.value.push(cloneAnnotation(activeAnnotation));
    }

    activeAnnotation = null;
    redrawCanvas();
  }

  function cancelAnnotation() {
    activeAnnotation = null;
    redrawCanvas();
  }

  function undoAnnotation() {
    annotations.value.pop();
    redrawCanvas();
  }

  function canvasDataUrl() {
    return canvasElement.value?.toDataURL("image/png") ?? null;
  }

  function redrawCanvas(previewAnnotation?: Annotation) {
    const canvas = canvasElement.value;
    const context = canvas?.getContext("2d");

    if (!canvas || !context || !screenshotImage) {
      return;
    }

    context.clearRect(0, 0, canvas.width, canvas.height);
    context.drawImage(screenshotImage, 0, 0);

    for (const annotation of annotations.value) {
      drawAnnotation(context, annotation);
    }

    if (previewAnnotation) {
      drawAnnotation(context, previewAnnotation);
    }
  }

  function canvasPosition(event: PointerEvent): Point | null {
    const canvas = canvasElement.value;

    if (!canvas) {
      return null;
    }

    const bounds = canvas.getBoundingClientRect();

    return {
      x: ((event.clientX - bounds.left) / bounds.width) * canvas.width,
      y: ((event.clientY - bounds.top) / bounds.height) * canvas.height,
    };
  }

  return {
    annotations,
    canvasElement,
    cancelAnnotation,
    canvasDataUrl,
    finishAnnotation,
    loadScreenshot,
    startAnnotation,
    undoAnnotation,
    updateAnnotation,
  };
}

function drawAnnotation(
  context: CanvasRenderingContext2D,
  annotation: Annotation,
) {
  context.strokeStyle = annotation.color;
  context.fillStyle = annotation.color;
  context.lineWidth = annotation.strokeWidth;
  context.lineCap = "round";
  context.lineJoin = "round";

  if (annotation.type === "rectangle") {
    drawRectangle(context, annotation);
  } else if (annotation.type === "arrow") {
    drawArrow(context, annotation);
  } else {
    drawFreehand(context, annotation);
  }
}

function drawRectangle(
  context: CanvasRenderingContext2D,
  annotation: Extract<Annotation, { type: "rectangle" }>,
) {
  context.strokeRect(
    annotation.start.x,
    annotation.start.y,
    annotation.end.x - annotation.start.x,
    annotation.end.y - annotation.start.y,
  );
}

function drawArrow(
  context: CanvasRenderingContext2D,
  annotation: Extract<Annotation, { type: "arrow" }>,
) {
  const angle = Math.atan2(
    annotation.end.y - annotation.start.y,
    annotation.end.x - annotation.start.x,
  );
  const headLength = Math.max(16, annotation.strokeWidth * 4);

  context.beginPath();
  context.moveTo(annotation.start.x, annotation.start.y);
  context.lineTo(annotation.end.x, annotation.end.y);
  context.stroke();

  context.beginPath();
  context.moveTo(annotation.end.x, annotation.end.y);
  context.lineTo(
    annotation.end.x - headLength * Math.cos(angle - Math.PI / 6),
    annotation.end.y - headLength * Math.sin(angle - Math.PI / 6),
  );
  context.lineTo(
    annotation.end.x - headLength * Math.cos(angle + Math.PI / 6),
    annotation.end.y - headLength * Math.sin(angle + Math.PI / 6),
  );
  context.closePath();
  context.fill();
}

function drawFreehand(
  context: CanvasRenderingContext2D,
  annotation: Extract<Annotation, { type: "draw" }>,
) {
  const [firstPoint, ...remainingPoints] = annotation.points;

  if (!firstPoint) {
    return;
  }

  context.beginPath();
  context.moveTo(firstPoint.x, firstPoint.y);

  for (const point of remainingPoints) {
    context.lineTo(point.x, point.y);
  }

  context.stroke();
}

function isMeaningfulAnnotation(annotation: Annotation) {
  if (annotation.type === "draw") {
    return annotation.points.length > 1;
  }

  return (
    Math.abs(annotation.end.x - annotation.start.x) > 3 ||
    Math.abs(annotation.end.y - annotation.start.y) > 3
  );
}

function cloneAnnotation(annotation: Annotation): Annotation {
  if (annotation.type === "draw") {
    return {
      ...annotation,
      points: annotation.points.map((point) => ({ ...point })),
    };
  }

  return {
    ...annotation,
    start: { ...annotation.start },
    end: { ...annotation.end },
  };
}
