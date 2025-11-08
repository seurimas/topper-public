import type { TimelineControl } from "$lib/combat/types";
import type { WasmTimeline, WasmTimeSlices } from "topper";

export type TimeRef = [HTMLElement, number];

export function scrollHandler(
    timelineControl: TimelineControl,
    timeRefs: Record<number, TimeRef>,
    boundingBoxes: Record<number, DOMRect>,
    timelineTime: number,
    setTimelineTime: (time: number) => void,
): void {
    if (timelineControl.type !== 'scrollLock') {
        return;
    }
    const { scrollY } = window;
    const scrollPoint = scrollY + window.innerHeight;
    let closestTime = 0;
    let closestDist = Infinity;
    for (const lineIdx in boundingBoxes) {
        const box = boundingBoxes[lineIdx];
        if (box.top > scrollPoint || box.bottom < scrollY) {
            continue;
        }
        const dist = Math.abs(box.top - scrollPoint);
        if (dist < closestDist) {
            closestDist = dist;
            const time = timeRefs[lineIdx][1];
            closestTime = time;
        }
    }
    if (closestTime !== timelineTime) {
        setTimelineTime(closestTime);
    }
};

export function getScrollPositionForTime(
    time: number,
    timeRefs: Record<number, TimeRef>,
    boundingBoxes: Record<number, DOMRect>,
): number {
    let closestLineIdx: number | null = null;
    let closestDist = Infinity;
    for (const lineIdx in timeRefs) {
        const refTime = timeRefs[lineIdx][1];
        if (refTime > time) {
            continue;
        }
        const dist = Math.abs(refTime - time);
        if (dist < closestDist) {
            closestDist = dist;
            closestLineIdx = parseInt(lineIdx);
        }
    }
    if (closestLineIdx === null) {
        return 0;
    }
    const box = boundingBoxes[closestLineIdx];
    return box.bottom - window.innerHeight;
}