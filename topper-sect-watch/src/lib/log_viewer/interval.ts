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
) {
    if (timelineControl.type !== 'timeStep' || timelineControl.speed <= 0 || timelineTime === undefined) {
        return;
    }
    const currentTime = timelineTime;
    const timeStep = 1000 / timelineControl.speed;
    let newTime = currentTime + timeStep;
    setTimelineTime(newTime);
    const scrollPosition = getScrollPositionForTime(newTime, timeRefs, boundingBoxes);
    window.scrollTo({ top: scrollPosition, behavior: 'instant' });
}