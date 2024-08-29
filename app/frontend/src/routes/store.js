import { writable } from "svelte/store";

export const count = writable(0);

export const settings = writable({
    candidates: {
        color: "auto",
        font_size: 16,
    },
    input_settings: {
        input_mode: '',
        tone_mode: '',
        t2: '',
        t3: '',
        t5: '',
        t6: '',
        t7: '',
        t8: '',
        t9: '',
        khin: '',
        hyphon: '',
        done: '',
    }
});
