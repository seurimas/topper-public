import type { TimelineControl } from "$lib/combat/types";
import { browser } from "$app/environment";

const voices = (browser && window.speechSynthesis) ? speechSynthesis.getVoices() : [];

let selectedVoice: string | undefined = $state(voices.length > 0 ? voices[0].name : undefined);

export const setSelectedVoice = (voice: string) => {
    selectedVoice = voice;
};
export const getSelectedVoice = () => {
    return selectedVoice;
};

let voiceSpeed: number = $state(2);

export const setVoiceSpeed = (speed: number) => {
    voiceSpeed = speed;
};
export const getVoiceSpeed = () => {
    return voiceSpeed;
};

// Actually, we'll only allow timeStep for now.
let validVoiceControlTypes: Set<TimelineControl['type']> = $state(new Set(['timeStep']));

export const isVoiceControlType = (type: TimelineControl['type']) => {
    return validVoiceControlTypes.has(type);
};
export const toggleVoiceControlType = (type: TimelineControl['type']) => {
    if (validVoiceControlTypes.has(type)) {
        validVoiceControlTypes.delete(type);
    } else {
        validVoiceControlTypes.add(type);
    }
};



const synth = browser && window.speechSynthesis;
export const ttsSpeak = (text: string) => {
    if (!synth) {
        return;
    }
    synth.cancel();
    ttsQueue(text);
};

export const ttsQueue = (text: string) => {
    if (selectedVoice === undefined || !synth) {
        return;
    }
    const utterThis = new SpeechSynthesisUtterance(text);
    utterThis.voice = voices.find(v => v.name === selectedVoice)!;
    utterThis.rate = voiceSpeed;
    synth.speak(utterThis);
};

export const speakIfEnabled = (text: string, controlType: TimelineControl['type']) => {
    if (validVoiceControlTypes.has(controlType)) {
        ttsSpeak(text);
    }
};