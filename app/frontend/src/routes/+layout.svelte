<script>
    import "beercss/dist/cdn/beer.min.css";
    import "./global.css";
    import Navbar from "../lib/Navbar.svelte";
    import "./store";

    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    onMount(async () => {
        await invoke("load_settings");
    })
</script>

<div>
    <Navbar />

    <main>
        <slot />
    </main>
</div>

<style>
    div {
        display: flex;
        flex-direction: row;
    }

    main {
        padding-left: 11rem;
    }
</style>
