export type ExplainerPage = {
    id: string;
    body: string[];
};

export const STORAGE_BUCKET_NAME = "sect_logs";

const I_WIN_REGEX = /^\((\w+)\)_(\w+)_vs_(\w+)_(\w+)_(\d+)_(\d+)$/;
const YOU_WIN_REGEX = /^(\w+)_(\w+)_vs_\((\w+)\)_(\w+)_(\d+)_(\d+)$/;
const DRAW_REGEX = /^(\w+)_(\w+)_vs_(\w+)_(\w+)_(\d+)_(\d+)$/;

export type SectLog = {
    name: string;
    myName: string;
    myClass: string;
    oppName: string;
    oppClass: string;
    length: number;
    duration?: number;
    winner: string;
};

export const parseLogId = (name: string): SectLog => {
        if (I_WIN_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length, duration] = I_WIN_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length),
                duration: parseInt(duration),
                winner: myName,
            };
        } else if (YOU_WIN_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length, duration] = YOU_WIN_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length),
                duration: parseInt(duration),
                winner: oppName,
            };
        } else if (DRAW_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length, duration] = DRAW_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length),
                duration: parseInt(duration),
                winner: 'draw',
            };
        }
        return { name, myName: 'unknown', myClass: 'unknown', oppName: 'unknown', oppClass: 'unknown', length: 0, winner: 'unknown' };
    };

const TIME_REGEX = /\[(\d\d):(\d\d):(\d\d):(\d\d)\]/;

export const parseTime = (timeStr: string) => {
    const match = TIME_REGEX.exec(timeStr);
    if (!match) return 0;
    const [, hours, minutes, seconds, centiseconds] = match;
    return (parseInt(hours) * 360000 + parseInt(minutes) * 6000 + parseInt(seconds) * 100 + parseInt(centiseconds));
};