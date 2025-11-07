export function recalculateBoundingBoxes(
    timeRefs: Record<number, [HTMLElement, number]>,
): Record<number, DOMRect> {
    const boundingBoxes: Record<number, DOMRect> = {};
    const { scrollX, scrollY } = window;
    for (const lineIdx in timeRefs) {
        const el = timeRefs[lineIdx][0];
        const { x, y, width, height } = el.getBoundingClientRect();
        boundingBoxes[lineIdx] = new DOMRect(x + scrollX, y + scrollY, width, height);
    }
    return boundingBoxes;
}

export function calculateBoundingBox(
    el: HTMLElement,
): DOMRect {
    const { scrollX, scrollY } = window;
    const { x, y, width, height } = el.getBoundingClientRect();
    return new DOMRect(x + scrollX, y + scrollY, width, height);
}