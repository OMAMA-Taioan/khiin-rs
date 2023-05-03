<script lang="ts">
    import "../app.css";
    import "../services/i18n";
    import Sidebar from "$lib/Sidebar.svelte";
    import Spinner from "$lib/Spinner.svelte";
    // import I18n from "$lib/i18n.svelte";

    import { invoke } from "@tauri-apps/api/tauri";
    // import { listen, type UnlistenFn } from '@tauri-apps/api/event'
    import { onMount, onDestroy } from "svelte";
    import { isLoading } from "svelte-i18n";

    // Settings type example
    type Settings = {
        font_size?: number;
        font?: string;
        dark_mode?: boolean;
    };

    export let settings: Settings = {};
    export let loaded = true;
    // export let unlisten: UnlistenFn;

    // Add window event listener
    async function updateSetting(settings: Settings) {
        settings = await invoke("updateSetting", {
            settings: JSON.stringify(settings),
        });
    }

    async function subscribe() {
        // unlisten = await listen('khiin-settings', (event: any) => {
        //     settings = JSON.parse(event)
        //     loaded = true
        // });
        loaded = true;
    }

    onMount(() => {
        // locale.set('zh-TW')
        subscribe();
    });

    onDestroy(() => {
        // This will probably never get called
        // unlisten()
    });
</script>

<div class="h-screen max-ww-[800px] flex bg-white">
    {#if loaded && !$isLoading}
        <Sidebar />
        <div class="flex min-h-fit w-full flex-col px-10 py-10">
            <slot />
        </div>
    {/if}
    {#if !loaded || $isLoading}
        <div class="flex h-full w-full justify-center items-center border">
            <Spinner />
        </div>
    {/if}
</div>
