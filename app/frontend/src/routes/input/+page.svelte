<script lang="ts">
    import { _ } from "svelte-i18n"    // import Settings from "../lib/Settings.svelte";
    import Toggle from "../../lib/Toggle.svelte"

    let double_hyphen_to_khin = false;
    let auto_capitalization = false;
    let convert_c_to_ch = false;
</script>

<h1 class="text-3xl mb-3">{$_('page.input.title')}</h1>

<div class="mt-8 max-w-md">
    <div class="grid grid-cols-1 gap-6">
        <label class="block">
            <span class="text-gray-700">{$_('page.input.default-input-mode')}</span>
            <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                <option>{$_('page.input.auto')}</option>
                <option>{$_('page.input.numeric')}</option>
                <option>{$_('page.input.telex')}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_('page.input.tone-mode')}</span>
            <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                <option>{$_('page.input.numeric')}</option>
                <option>{$_('page.input.telex')}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_('page.input.temporarily-disable')}</span>
            <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                <option>{$_('page.input.shift')}</option>
                <option>{$_('page.input.ctrl-space')}</option>
            </select>
        </label>
        <label class="block">
            <span class="text-gray-700">{$_('page.input.switch-mode')}</span>
            <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                <option>{$_('page.input.alt-backtick')}</option>
                <option>{$_('page.input.alt-space')}</option>
            </select>
        </label>
        <label class="inline-flex items-center mt-4">
            <Toggle bind:checked={double_hyphen_to_khin} />
            <span class="ml-2 text-gray-700">{$_('page.input.convert-double-hyphen')}</span>
        </label>
        <label class="inline-flex items-center">
            <Toggle bind:checked={convert_c_to_ch} />
            <span class="ml-2 text-gray-700">{$_('page.input.convert-c-to-ch')}</span>
        </label>

        <label class="inline-flex items-center">
            <Toggle bind:checked={auto_capitalization} />
            <span class="ml-2 text-gray-700">{$_('page.input.auto-capitalization')}</span>
        </label>
    </div>
</div>