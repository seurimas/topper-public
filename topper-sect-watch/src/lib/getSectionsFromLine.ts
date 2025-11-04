export type LineSection = { text: string, color: string, time?: number };

export const getSectionsFromLine = (line: string) => {
    const parts: LineSection[] = [];
    let remaining = line;
    let current = '';
    let color = 'white';
    while (remaining.length > 0) {
        // Parse a string like: <color>text<color2>text2 -> [{text, color}, {text2, color2}]
        const match = remaining.match(/<([a-zA-Z0-9#]+)>/);
        if (match) {
            if (match.index! > 0) {
                current += remaining.slice(0, match.index);
            }
            if (current.length > 0) {
                parts.push({ text: current, color });
                current = '';
            }
            color = match[1];
            remaining = remaining.slice(match.index! + match[0].length);
        } else {
            current += remaining;
            remaining = '';
            if (current.length > 0) {
                parts.push({ text: current, color });
            }
        }
    }
    if (parts.length === 0) {
        return parts;
    }
    const finalPart = parts[parts.length - 1];
    const timeMatch = finalPart.text.match(/\[(\d\d):(\d\d):(\d\d):(\d\d)]/);
    if (timeMatch) {
        // We have a time slice at the end of the line. We need to create a ref for it.
        const [timeText, hh, mm, ss, cc] = timeMatch;
        const time = parseInt(hh) * 360000 + parseInt(mm) * 6000 + parseInt(ss) * 100 + parseInt(cc);
        const beforeTime = finalPart.text.slice(0, timeMatch.index);
        if (beforeTime.length > 0) {
            finalPart.text = beforeTime;
            parts.push({ text: timeText, color: finalPart.color, time } );
        } else {
            // The entire final part is just the time slice.
            finalPart.time = time;
        }
    }
    return parts;
};