import type { TimelineControl } from "$lib/combat/types";
import type { WasmTimeline, WasmTimeSlices } from "topper";
import { getScrollPositionForTime, type TimeRef } from "./scroll";

export const FRAME_RATE = 30;
export const FRAME_INTERVAL = 1000 / FRAME_RATE;

export function intervalHandler(
    timelineControl: TimelineControl,
    timeRefs: Record<number, TimeRef>,
    boundingBoxes: Record<number, DOMRect>,
    timelineTime: number,
    setTimelineTime: (time: number) => void,
    deltaTime: number,
) {
    if (timelineControl.type !== 'timeStep' || timelineControl.speed <= 0 || timelineTime === undefined) {
        return;
    }
    const currentTime = timelineTime;
    // Timeline time is in centiseconds, so we divide by 10 to convert milliseconds to centiseconds
    const timeStep = deltaTime * timelineControl.speed / 10;
    let newTime = currentTime + timeStep;
    setTimelineTime(newTime);
    const scrollPosition = getScrollPositionForTime(newTime, timeRefs, boundingBoxes);
    window.scrollTo({ top: scrollPosition, behavior: 'instant' });
}