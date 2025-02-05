<script lang="ts">
    import { _ } from "svelte-i18n"    // import Settings from "../lib/Settings.svelte";
    
    // Add version information variable
    const version = "v0.2.4"; // Replace with actual version number
</script>

<h1 class="text-3xl mb-3">{$_('page.home.title')}</h1>
<p class="mb-3 text-gray-500">Perfect for learning how the framework works, prototyping a new idea, or creating a demo to share online.</p>
<img src="/app-icon.png" class="w-10 h-10" alt="Khíín 台語打字法" />

<!-- Add version information -->
<div class="fixed bottom-2 right-2 text-sm text-gray-500 m-2">
    {version}
</div>

