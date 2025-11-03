export type ExplainerPage = {
    id: string;
    body: string[];
};

export const STORAGE_BUCKET_NAME = "sect_logs";

const I_WIN_REGEX = /^\((\w+)\)_(\w+)_vs_(\w+)_(\w+)_(\d+)$/;
const YOU_WIN_REGEX = /^(\w+)_(\w+)_vs_\((\w+)\)_(\w+)_(\d+)$/;
const DRAW_REGEX = /^(\w+)_(\w+)_vs_(\w+)_(\w+)_(\d+)$/;

export const parseLogId = (name: string) => {
        if (I_WIN_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length] = I_WIN_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length),
                winner: myName,
            };
        } else if (YOU_WIN_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length] = YOU_WIN_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length),
                winner: oppName,
            };
        } else if (DRAW_REGEX.test(name)) {
            const [, myName, myClass, oppName, oppClass, length1,] = DRAW_REGEX.exec(name)!;
            return {
                name,
                myName,
                myClass,
                oppName,
                oppClass,
                length: parseInt(length1), // both lengths are the same in a draw
                winner: 'draw',
            };
        }
        return { name, myName: 'unknown', myClass: 'unknown', oppName: 'unknown', oppClass: 'unknown', length: 0, winner: 'unknown' };
    };