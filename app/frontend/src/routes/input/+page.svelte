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
    let tone_mode_disabled = false
    $: if (input_mode == "auto") {
		tone_mode_disabled = true;
	} else {
        tone_mode_disabled = false;
    }

    async function toneModeChanged(event) {
        const new_tone_mode = event.target.value;
        settings.update(settings => {
            settings.input_settings.tone_mode = new_tone_mode;
            return settings;
        })
        await updateSettings();
    }

    async function inputModeChanged(event) {
        const new_input_mode = event.target.value;
        settings.update(settings => {
            settings.input_settings.input_mode = new_input_mode;
            return settings;
        })

        await updateSettings();
    }

    async function outputModeChanged(event) {
        const new_output_mode = event.target.value;
        settings.update(settings => {
            settings.input_settings.output_mode = new_output_mode;
            return settings;
        })
        await updateSettings();
    }

    async function updateSettings() {
        try {
            await invoke('update_settings', { settings: JSON.stringify($settings) });
        } catch (error) {
            console.error('Failed to update settings:', error);
        }
    }
</script>

<style>
    select[disabled] {
        color: #AAA;
    } 
</style>
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
                <option value="auto">{$_("page.input.auto")}</option>
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
            <select
                class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50"
            >
                <option>{$_("page.input.alt-backtick")}</option>
            </select>
        </label>
        <!-- <label class="inline-flex items-center mt-4">
            <Toggle bind:checked={double_hyphen_to_khin} />
            <span class="ml-2 text-gray-700"
                >{$_("page.input.convert-double-hyphen")}</span
            >
        </label>
        <label class="inline-flex items-center">
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
