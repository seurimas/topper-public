import { describe, expect, it } from "vitest";
import { getSectionsFromLine } from "./getSectionsFromLine";

describe('getSectionsFromLine', () => {
    it('parses a line with time section correctly', () => {
        const line = "<#00cd00>H:4160/4160 <#005fff>M:6344 <#ff00ff>P:3764 <#e5e5e5>[cs][ebs] <#00ffff>Morning<#ffffff> [02:30:47:52]";
        const sections = getSectionsFromLine(line);
        expect(sections).toEqual([
            { text: "H:4160/4160 ", color: "#00cd00" },
            { text: "M:6344 ", color: "#005fff" },
            { text: "P:3764 ", color: "#ff00ff" },
            { text: "[cs][ebs] ", color: "#e5e5e5" },
            { text: "Morning", color: "#00ffff" },
            { text: " ", color: "#ffffff" },
            { text: "[02:30:47:52]", color: "#ffffff", timeSection: true }
        ]);
    });

    it('parses a line without time section correctly', () => {
        const line = "<#ffff00>>>> <white>qeb stand;;order hellcat attack Naivara;;enact zenith";
        const sections = getSectionsFromLine(line);
        expect(sections).toEqual([
            { text: ">>> ", color: "#ffff00" },
            { text: "qeb stand;;order hellcat attack Naivara;;enact zenith", color: "white" }
        ]);
    });
});