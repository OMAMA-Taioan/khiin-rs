<script lang="ts">
    import { _ } from "svelte-i18n"; // import Settings from "../lib/Settings.svelte";
    import Toggle from "../../lib/Toggle.svelte";
    import { settings, count } from "../store.js";
    import { invoke } from "@tauri-apps/api/tauri";

    let double_hyphen_to_khin = false;
    let auto_capitalization = false;
    let convert_c_to_ch = false;
    let input_mode = $settings.input_settings.input_mode;
    let tone_mode = $settings.input_settings.tone_mode;
    let output_mode = $settings.input_settings.output_mode;
    let khin_mode = $settings.input_settings.khin_mode;
    let mode_shortcut = $settings.input_settings.input_mode_shortcut;
    let tone_mode_disabled = false;

    // Telex key settings
    let t2_key = $settings.input_settings.t2;
    let t3_key = $settings.input_settings.t3;
    let t5_key = $settings.input_settings.t5;
    let t6_key = $settings.input_settings.t6;
    let t7_t8_key = $settings.input_settings.t7;
    let t9_key = $settings.input_settings.t9;
    let hyphen_key = $settings.input_settings.hyphen;
    let khin_key = $settings.input_settings.khin;
    let done_key = $settings.input_settings.done;

    // Available keys for Telex: SFLJXW + DVR + YQZ
    const allKeys = [
        "s",
        "f",
        "l",
        "j",
        "x",
        "w",
        "d",
        "v",
        "r",
        "y",
        "q",
        "z",
    ];

    $: currentAssignments = {
        t2: t2_key,
        t3: t3_key,
        t5: t5_key,
        t6: t6_key,
        t7_t8: t7_t8_key,
        t9: t9_key,
        hyphen: hyphen_key,
        khin: khin_key,
        done: done_key,
    };

    $: allUsedKeys = Object.values(currentAssignments).filter((k) => k);

    function getOptionsFor(fieldKey: string, _dependencies?: any) {
        const myCurrentValue = currentAssignments[fieldKey];

        return allKeys.filter((key) => {
            const isUsed = allUsedKeys.includes(key);
            const isMyValue = key === myCurrentValue;
            return !isUsed || isMyValue;
        });
    }

    $: if (input_mode == "auto") {
        tone_mode_disabled = true;
    } else {
        tone_mode_disabled = false;
    }

    async function toneModeChanged(event) {
        const new_tone_mode = event.target.value;
        settings.update((settings) => {
            settings.input_settings.tone_mode = new_tone_mode;
            return settings;
        });
        await updateSettings();
    }

    async function inputModeChanged(event) {
        const new_input_mode = event.target.value;
        settings.update((settings) => {
            settings.input_settings.input_mode = new_input_mode;
            return settings;
        });

        await updateSettings();
    }

    async function outputModeChanged(event) {
        const new_output_mode = event.target.value;
        settings.update((settings) => {
            settings.input_settings.output_mode = new_output_mode;
            return settings;
        });
        await updateSettings();
    }

    async function khinModeChanged(event) {
        const new_khin_mode = event.target.value;
        settings.update((settings) => {
            settings.input_settings.khin_mode = new_khin_mode;
            return settings;
        });
        await updateSettings();
    }

    async function modeShortcutChanged(event) {
        const new_mode_shortcut = event.target.value;
        settings.update((settings) => {
            settings.input_settings.input_mode_shortcut = new_mode_shortcut;
            return settings;
        });
        await updateSettings();
    }

    async function keySettingChanged(field: string, event) {
        const newValue = event.target.value;

        settings.update((settings) => {
            switch (field) {
                case "t2":
                    settings.input_settings.t2 = newValue;
                    break;
                case "t3":
                    settings.input_settings.t3 = newValue;
                    break;
                case "t5":
                    settings.input_settings.t5 = newValue;
                    break;
                case "t6":
                    settings.input_settings.t6 = newValue;
                    break;
                case "t7_t8":
                    settings.input_settings.t7 = newValue;
                    settings.input_settings.t8 = newValue;
                    break;
                case "t9":
                    settings.input_settings.t9 = newValue;
                    break;
                case "hyphen":
                    settings.input_settings.hyphen = newValue;
                    break;
                case "khin":
                    settings.input_settings.khin = newValue;
                    break;
                case "done":
                    settings.input_settings.done = newValue;
                    break;
            }
            return settings;
        });
        await updateSettings();
    }

    async function updateSettings() {
        try {
            await invoke("update_settings", {
                settings: JSON.stringify($settings),
            });
        } catch (error) {
            console.error("Failed to update settings:", error);
        }
    }
</script>

<h1 class="text-3xl mb-3">{$_("page.input.title")}</h1>
<div class="mt-8 max-w-md">
    <div class="grid grid-cols-1 gap-6">
        <label class="block">
            <span class="text-gray-700"
                >{$_("page.input.default-input-mode")}</span
            >
            <select
                bind:value={input_mode}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={inputModeChanged}
            >
                <option value="classic">{$_("page.input.classic")}</option>
                <option value="manual">{$_("page.input.manual")}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_("page.input.tone-mode")}</span>
            <select
                bind:value={tone_mode}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                disabled={tone_mode_disabled}
                on:change={toneModeChanged}
            >
                <option value="numeric">{$_("page.input.numeric")}</option>
                <option value="telex">{$_("page.input.telex")}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_("page.input.output-mode")}</span>
            <select
                bind:value={output_mode}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={outputModeChanged}
            >
                <option value="lomaji">{$_("page.input.lomaji")}</option>
                <option value="hanji">{$_("page.input.hanji")}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_("page.input.khin-mode")}</span>
            <select
                bind:value={khin_mode}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={khinModeChanged}
            >
                <option value="khinless">{$_("page.input.khinless")}</option>
                <option value="hyphen">--</option>
                <option value="dot"> ·</option>
            </select>
        </label>
        <!-- <label class="block">
            <span class="text-gray-700"
                >{$_("page.input.temporarily-disable")}</span
            >
            <select
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
            >
                <option>{$_("page.input.shift")}</option>
                <option>{$_("page.input.ctrl-space")}</option>
            </select>
        </label> -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.switch-mode")}</span>
            {#await invoke("is_windows") then is_windows_os}
                <select
                    bind:value={mode_shortcut}
                    class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                    on:change={modeShortcutChanged}
                >
                    {#if is_windows_os}
                        <option value="default"
                            >{$_("page.input.ctrl-backtick")}</option
                        >
                        <option value="shift">{$_("page.input.shift")}</option>
                    {:else}
                        <option value="default"
                            >{$_("page.input.alt-backtick")}</option
                        >
                    {/if}
                </select>
            {/await}
        </label>
        <!-- <label class="inline-flex items-center">
            <Toggle bind:checked={convert_c_to_ch} />
            <span class="ml-2 text-gray-700"
                >{$_("page.input.convert-c-to-ch")}</span
            >
        </label>

        <label class="inline-flex items-center">
            <Toggle bind:checked={auto_capitalization} />
            <span class="ml-2 text-gray-700"
                >{$_("page.input.auto-capitalization")}</span
            >
        </label> -->
    </div>
</div>

<!-- Telex Key Settings Section -->
<div class="mt-8 max-w-md">
    <h2 class="text-xl font-semibold mb-4 text-gray-700">
        {$_("page.input.telex-key-settings")}
    </h2>
    <div class="grid grid-cols-2 gap-4">
        <!-- T2 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t2-key")}</span>
            <select
                bind:value={t2_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t2", e)}
            >
                {#each getOptionsFor("t2", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- T3 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t3-key")}</span>
            <select
                bind:value={t3_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t3", e)}
            >
                {#each getOptionsFor("t3", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- T5 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t5-key")}</span>
            <select
                bind:value={t5_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t5", e)}
            >
                {#each getOptionsFor("t5", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- T6 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t6-key")}</span>
            <select
                bind:value={t6_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t6", e)}
            >
                {#each getOptionsFor("t6", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- T7/T8 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t7-t8-key")}</span>
            <select
                bind:value={t7_t8_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t7_t8", e)}
            >
                {#each getOptionsFor("t7_t8", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- T9 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.t9-key")}</span>
            <select
                bind:value={t9_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("t9", e)}
            >
                {#each getOptionsFor("t9", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- Khin 輕 -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.khin-key")}</span>
            <select
                bind:value={khin_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("khin", e)}
            >
                {#each getOptionsFor("khin", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>
    </div>
</div>

<div class="mt-8 max-w-md">
    <div class="grid grid-cols-2 gap-4">
        <!-- Hyphen -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.hyphen-key")}</span>
            <select
                bind:value={hyphen_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("hyphen", e)}
            >
                {#each getOptionsFor("hyphen", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>

        <!-- Done (end syllable) -->
        <label class="block">
            <span class="text-gray-700">{$_("page.input.done-key")}</span>
            <select
                bind:value={done_key}
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
                on:change={(e) => keySettingChanged("done", e)}
            >
                {#each getOptionsFor("done", allUsedKeys) as key}
                    <option value={key}>{key.toUpperCase()}</option>
                {/each}
            </select>
        </label>
    </div>
</div>
<br>
<style>
    select[disabled] {
        color: #aaa;
    }
</style>
