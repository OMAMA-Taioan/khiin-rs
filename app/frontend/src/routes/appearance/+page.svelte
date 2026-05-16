<script lang="ts">
    import { _, locale } from "svelte-i18n" 
    import { settings } from "../store";
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Add version information variable
    const version = "v0.3.1"; // Replace with actual version number

    async function updateLanguage(event: Event) {
        const target = event.target as HTMLSelectElement;
        locale.set(target.value);
        $settings.appearance.locale = target.value;
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

<h1 class="text-3xl mb-3">{$_('page.appearance.title')}</h1>

<div class="mt-8 max-w-md">
    <div class="grid grid-cols-1 gap-6">
        <!-- <label class="block">
          <span class="text-gray-700">{$_('page.appearance.theme')}</span>
          <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
            <option>{$_('page.appearance.auto')}</option>
            <option>{$_('page.appearance.light')}</option>
            <option>{$_('page.appearance.dark')}</option>
          </select>
        </label> -->
        <label class="block">
          <span class="text-gray-700">{$_('page.appearance.language')}</span>
          <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50" bind:value={$settings.appearance.locale} on:change={updateLanguage}>
            <option value="en">English</option>
            <option value="oan_Han">漢羅</option>
            <option value="oan_Latn">Lômájī</option>
          </select>
        </label>
        <label class="block">
          <span class="text-gray-700">{$_('page.appearance.font-size')}</span>
          <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50" bind:value={$settings.candidates.font_size} on:change={updateSettings}>
            <option value={20}>{$_('page.appearance.lg')}</option>
            <option value={16}>{$_('page.appearance.sm')}</option>
          </select>
        </label>
    </div>
</div>
<!-- Add version information -->
<div class="fixed bottom-2 right-2 text-sm text-gray-500 m-2">
    {version}
</div>
<!-- - Theme (auto, dark, light)
- UI language (english, hanlo, or lomaji)
- Candidate font size (sliding scale from 12 to 32 would be ok, or something similar)
- Candidate font (opening system font picker and select one) -->
