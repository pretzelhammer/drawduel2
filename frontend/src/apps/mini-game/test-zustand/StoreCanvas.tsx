import { useEffect, useRef, useState, type FC } from 'react';
import { StoreState, useStore } from './TestZustandAppStore';
import classes from './StoreCanvas.module.css';

function drawSmoothLine(
    context: CanvasRenderingContext2D,
    x1: number,
    y1: number,
    x2: number,
    y2: number,
) {
    context.beginPath();
    context.moveTo(x1, y1);
    context.lineTo(x2, y2);
    context.stroke();
}

function drawPixelatedLine(
    context: CanvasRenderingContext2D,
    pixelSize: number,
    x1: number,
    y1: number,
    x2: number,
    y2: number,
) {
    const dx = Math.abs(x2 - x1);
    const dy = Math.abs(y2 - y1);
    const sx = x1 < x2 ? 1 : -1;
    const sy = y1 < y2 ? 1 : -1;
    let err = dx - dy;

    while (true) {
        context.fillRect(
            Math.floor(x1 / pixelSize) * pixelSize,
            Math.floor(y1 / pixelSize) * pixelSize,
            pixelSize,
            pixelSize,
        );

        if (x1 === x2 && y1 === y2) break;
        const e2 = err * 2;
        if (e2 > -dy) {
            err -= dy;
            x1 += sx;
        }
        if (e2 < dx) {
            err += dx;
            y1 += sy;
        }
    }
}

function drawAntiAliasedPixelatedLine(
    context: CanvasRenderingContext2D,
    pixelSize: number,
    x1: number,
    y1: number,
    x2: number,
    y2: number,
) {
    const dx = Math.abs(x2 - x1);
    const dy = Math.abs(y2 - y1);
    const sx = x1 < x2 ? 1 : -1;
    const sy = y1 < y2 ? 1 : -1;
    let err = dx - dy;

    while (true) {
        drawAntiAliasedPixel(context, pixelSize, x1, y1);

        if (x1 === x2 && y1 === y2) break;
        const e2 = err * 2;
        if (e2 > -dy) {
            err -= dy;
            x1 += sx;
        }
        if (e2 < dx) {
            err += dx;
            y1 += sy;
        }
    }
}

function drawAntiAliasedPixel(
    context: CanvasRenderingContext2D,
    pixelSize: number,
    x: number,
    y: number,
) {
    context.fillRect(
        Math.floor(x / pixelSize) * pixelSize,
        Math.floor(y / pixelSize) * pixelSize,
        pixelSize,
        pixelSize,
    );

    // Add anti-aliasing around the edges
    context.globalAlpha = 0.05;
    context.fillRect(
        Math.floor(x / pixelSize) * pixelSize - pixelSize,
        Math.floor(y / pixelSize) * pixelSize,
        pixelSize,
        pixelSize,
    );
    context.fillRect(
        Math.floor(x / pixelSize) * pixelSize + pixelSize,
        Math.floor(y / pixelSize) * pixelSize,
        pixelSize,
        pixelSize,
    );
    context.fillRect(
        Math.floor(x / pixelSize) * pixelSize,
        Math.floor(y / pixelSize) * pixelSize - pixelSize,
        pixelSize,
        pixelSize,
    );
    context.fillRect(
        Math.floor(x / pixelSize) * pixelSize,
        Math.floor(y / pixelSize) * pixelSize + pixelSize,
        pixelSize,
        pixelSize,
    );
    context.globalAlpha = 1.0;
}

// https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
const contextSettings: CanvasRenderingContext2DSettings = {
    alpha: false,
    colorSpace: 'srgb',
    desynchronized: true, // lets see if this does anything
};

function cursorStyle(brushSize: number, brushColor: string): string {
    // Create an SVG string
    const svg = `
        <svg xmlns="http://www.w3.org/2000/svg" width="${brushSize}" height="${brushSize}" viewBox="0 0 ${brushSize} ${brushSize}">
            <circle cx="${brushSize / 2}" cy="${brushSize / 2}" r="${
        brushSize / 2 - 1
    }" fill="${brushColor}" stroke="${brushColor}" stroke-width="1" />
        </svg>
    `;

    // Encode the SVG to a data URL
    const svgDataUrl = `data:image/svg+xml;base64,${btoa(svg)}`;

    // Update the canvas cursor
    return `url(${svgDataUrl}) ${brushSize / 2} ${brushSize / 2}, auto`;
}

export type LineType = 'smooth' | 'pixelated' | 'antialiased-pixelated';

export type CanvasMode = 'draw' | 'view';

export interface CanvasV5Props {
    lineType: LineType;
    mode: CanvasMode;
}

interface InternalRef {
    // from store
    drawOps: DrawOperation[];
    addDrawOp: (drawOp: DrawOperation) => void;
    // local state
    brushSize: number;
    primaryColor: string;
    secondaryColor: string;
    lineType: LineType;
    isDrawing: boolean;
    drewOps: number;
    lastX: number;
    lastY: number;
    mode: CanvasMode;
    redrawCanvas: () => void;
}

export enum DrawAction {
    StartStroke = 0,
    ContinueStroke = 1,
    EndStroke = 2,
}

export type DrawOperation = [DrawAction, number, number];

export const StoreCanvas: FC<CanvasV5Props> = (props: CanvasV5Props) => {
    // const { clientContext, dispatchClientEvent, setClientState } =
    //     useClientContext();
    // const { drawOps, addDrawOp } = useStore((state: StoreState) => ({
    //     drawOps: state.drawOps,
    //     addDrawOp: state.addDrawOp,
    // }));
    const drawOps = useStore((state) => state.drawOps);
    const addDrawOp = useStore((state) => state.addDrawOp);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [brush, setBrush] = useState({
        size: 5,
        primaryColor: '#000000',
        secondaryColor: '#ffffff',
    });

    // only set once
    const internalRef = useRef<InternalRef>({
        drawOps,
        addDrawOp,
        brushSize: 5,
        primaryColor: '#000000',
        secondaryColor: '#ffffff',
        lineType: props.lineType,
        mode: props.mode,
        isDrawing: false,
        drewOps: 0,
        lastX: 0,
        lastY: 0,
        redrawCanvas: () => {},
    });

    // re-set on follow-up re-renders
    internalRef.current.drawOps = drawOps;
    internalRef.current.addDrawOp = addDrawOp;
    if (internalRef.current.lineType !== props.lineType) {
        internalRef.current.lineType = props.lineType;
        internalRef.current.drewOps = 0;
    }

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        function onResize() {
            const internal = internalRef.current;
            const newWidth = canvas!.parentElement!.clientWidth;
            const newHeight = (newWidth * 3) / 4; // 4:3 aspect ratio
            canvas!.width = newWidth;
            canvas!.height = newHeight;
            internal.drewOps = 0;
            redrawCanvas();
        }

        function startDrawing(e: MouseEvent) {
            const internal = internalRef.current;
            internal.isDrawing = true;
            const x = e.offsetX / canvas!.width;
            const y = e.offsetY / canvas!.height;
            const drawOp: DrawOperation = [DrawAction.StartStroke, x, y];
            // internal.setClientState({
            //     ...internal.clientContext.clientState,
            //     draw: [...internal.clientContext.clientState.draw, drawOp],
            // });
            internal.addDrawOp(drawOp);
        }

        function draw(e: MouseEvent) {
            const internal = internalRef.current;
            if (!internal.isDrawing) return;
            const x = e.offsetX / canvas!.width;
            const y = e.offsetY / canvas!.height;
            const drawOp: DrawOperation = [DrawAction.ContinueStroke, x, y];
            // internal.setClientState({
            //     ...internal.clientContext.clientState,
            //     draw: [...internal.clientContext.clientState.draw, drawOp],
            // });
            internal.addDrawOp(drawOp);
            redrawCanvas();
        }

        function stopDrawing() {
            const internal = internalRef.current;
            internal.isDrawing = false;
            // const drawOp = [DrawAction.EndStroke];
            // internal.setClientState({
            //     ...internal.clientContext.clientState,
            //     draw: [...internal.clientContext.clientState.draw, drawOp],
            // });
        }

        function redrawCanvas() {
            const internal = internalRef.current;
            const context = canvas!.getContext('2d')!;
            const canvasWidth = canvas!.width;
            const canvasHeight = canvas!.height;

            if (internal.drewOps === 0) {
                context.clearRect(0, 0, canvasWidth, canvasHeight);
                context.fillStyle = 'white';
                context.fillRect(0, 0, canvasWidth, canvasHeight);
            }

            context.lineCap = 'round';
            context.lineJoin = 'round';
            context.lineWidth = brush.size;
            context.strokeStyle = brush.primaryColor;
            context.fillStyle = brush.secondaryColor;
            const pixelSize = brush.size;

            canvasRef.current!.style.cursor = cursorStyle(
                internal.brushSize,
                internal.primaryColor,
            );

            const drawOps = internal.drawOps;

            for (let i = internal.drewOps; i < drawOps.length; i++) {
                const drawOp = drawOps[i];
                let targetX1 = 0;
                let targetY1 = 0;
                let targetX2 = 0;
                let targetY2 = 0;
                if (drawOp[0] === DrawAction.StartStroke) {
                    targetX1 = Math.round(drawOp[1] * canvasWidth);
                    targetY1 = Math.round(drawOp[2] * canvasHeight);
                    targetX2 = targetX1;
                    targetY2 = targetY1;
                } else if (drawOp[0] === DrawAction.ContinueStroke) {
                    const prevDrawOp = drawOps[i - 1];
                    // assume it is a start or continue stroke
                    targetX1 = Math.round(prevDrawOp[1] * canvasWidth);
                    targetY1 = Math.round(prevDrawOp[2] * canvasHeight);
                    targetX2 = Math.round(drawOp[1] * canvasWidth);
                    targetY2 = Math.round(drawOp[2] * canvasHeight);
                } else if (drawOp[0] === DrawAction.EndStroke) {
                    // no-op
                    continue;
                }

                if (internal.lineType === 'smooth') {
                    drawSmoothLine(
                        context,
                        targetX1,
                        targetY1,
                        targetX2,
                        targetY2,
                    );
                } else if (internal.lineType === 'pixelated') {
                    drawPixelatedLine(
                        context,
                        pixelSize,
                        targetX1,
                        targetY1,
                        targetX2,
                        targetY2,
                    );
                } else if (internal.lineType === 'antialiased-pixelated') {
                    drawAntiAliasedPixelatedLine(
                        context,
                        pixelSize,
                        targetX1,
                        targetY1,
                        targetX2,
                        targetY2,
                    );
                }
            }
            internal.drewOps = drawOps.length;
        }

        internalRef.current.redrawCanvas = redrawCanvas;

        if (internalRef.current.mode === 'draw') {
            canvas.addEventListener('mousedown', startDrawing);
            canvas.addEventListener('mousemove', draw);
            canvas.addEventListener('mouseup', stopDrawing);
            canvas.addEventListener('mouseout', stopDrawing);
        }
        window.addEventListener('resize', onResize);

        onResize();

        return () => {
            canvas.removeEventListener('mousedown', startDrawing);
            canvas.removeEventListener('mousemove', draw);
            canvas.removeEventListener('mouseup', stopDrawing);
            canvas.removeEventListener('mouseout', stopDrawing);
            window.removeEventListener('resize', onResize);
        };
    }, [canvasRef.current, props.mode, brush]);

    internalRef.current.redrawCanvas();

    console.log({ classes });

    return (
        <div>
            <canvas
                ref={canvasRef}
                style={{ cursor: cursorStyle(brush.size, brush.primaryColor) }}
            />
            <div className={classes.controls}>
                <div
                    className={classes['preview-color']}
                    style={
                        {
                            '--color': brush.primaryColor,
                        } as React.CSSProperties
                    }
                />
                <div
                    className={classes['preview-color']}
                    style={
                        {
                            '--color': brush.secondaryColor,
                        } as React.CSSProperties
                    }
                />
                <div
                    className={classes['preview-color']}
                    style={{ '--color': '#000000' } as React.CSSProperties}
                    onClick={() =>
                        setBrush({ ...brush, primaryColor: '#000000' })
                    }
                />
                <div
                    className={classes['preview-color']}
                    style={{ '--color': '#ff0000' } as React.CSSProperties}
                    onClick={() =>
                        setBrush({ ...brush, primaryColor: '#ff0000' })
                    }
                />
            </div>
        </div>
    );
};
