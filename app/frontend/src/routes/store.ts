import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

import { writable } from "svelte/store";

type RecursivePartial<T> = {
    [P in keyof T]?: RecursivePartial<T[P]>;
};

interface AppSettings {
    candidates?: {
        color?: string;
        font_size?: number;
    };
    input_settings?: {
        input_mode?: string;
        tone_mode?: string;
        t2?: string;
        t3?: string;
        t5?: string;
        t6?: string;
        t7?: string;
        t8?: string;
        t9?: string;
        khin?: string;
    };
}

export const settings = writable<AppSettings>({
    candidates: {
        color: "auto",
        font_size: 16,
    },
});

export const updateSettings = (updates: AppSettings) => {
    invoke("update_settings", { settings: JSON.stringify(updates) });
};

const unlisten = await appWindow.listen<AppSettings>(
    "update_settings",
    (event) => {
        console.log(event);
        settings.set(event.payload);
    },
);
