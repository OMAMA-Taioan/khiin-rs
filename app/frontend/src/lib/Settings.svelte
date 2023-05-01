<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let settings = {};

	async function getSettings() {
		settings = await invoke('load_settings');
	}
</script>

<div>
    <button on:click={getSettings}>Load</button>
	<p>{JSON.stringify(settings)}</p>
</div>
