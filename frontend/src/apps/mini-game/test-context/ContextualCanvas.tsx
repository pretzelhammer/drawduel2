// import { type FunctionalComponent, Component, createRef, type RefObject } from 'preact';
// import { useEffect, useRef } from 'preact/hooks';
// import { useClientContext, ClientContextKey } from '../../game/context/ClientContextProvider';
// import { type ClientContext, type ClientState } from 'src/frontend/game/context/clientContext';
// import { type ClientEvent } from 'src/agnostic/events';

import { useContext, useEffect, useRef, type FC } from 'react';
import { TestAppContext, type TestAppState } from './TestContextAppContext';

/**
 * my fourth attempt at a canvas component
 * THE GOOD
 * - same as third attempt, now as a functional
 *   component tho instead of a class component
 * - draw ops also refactored
 */

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

/**
 * idk how this works, i asked chatGPT to generate
 * this function and it delivered
 */
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

/**
 * idk how this works, i asked chatGPT to generate
 * this function and it delivered
 */
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

/**
 * idk how this works, i asked chatGPT to generate
 * this function and it delivered
 */
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

export type LineType = 'smooth' | 'pixelated' | 'antialiased-pixelated';

export type CanvasMode = 'draw' | 'view';

export interface CanvasV5Props {
    lineType: LineType;
    mode: CanvasMode;
}

interface InternalRef {
    // from context
    state: TestAppState;
    setState: (state: TestAppState) => void;
    // local state
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

export const CanvasV5: FC<CanvasV5Props> = (props: CanvasV5Props) => {
    // const { clientContext, dispatchClientEvent, setClientState } =
    //     useClientContext();
    const { state, setState } = useContext(TestAppContext);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    // only set once
    const internalRef = useRef<InternalRef>({
        state,
        setState,
        lineType: props.lineType,
        mode: props.mode,
        isDrawing: false,
        drewOps: 0,
        lastX: 0,
        lastY: 0,
        redrawCanvas: () => {},
    });

    // re-set on follow-up re-renders
    internalRef.current.state = state;
    internalRef.current.setState = setState;
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
            internal.setState({
                ...internal.state,
                drawOps: [...internal.state.drawOps, drawOp],
            });
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
            internal.setState({
                ...internal.state,
                drawOps: [...internal.state.drawOps, drawOp],
            });
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

            const thickness = 5;
            context.lineCap = 'round';
            context.lineJoin = 'round';
            context.lineWidth = thickness;
            context.strokeStyle = 'black';
            context.fillStyle = 'black';
            const pixelSize = thickness;

            const drawOps = internal.state.drawOps;

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
    }, [canvasRef.current, props.mode]);

    internalRef.current.redrawCanvas();

    return <canvas ref={canvasRef} />;
};
