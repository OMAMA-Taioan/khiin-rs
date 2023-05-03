<script lang="ts">
    export let checked: boolean = false;
    function toggle() {
        checked = !checked;
    }
</script>
<div class="w-10 h-6 flex items-center {checked ? 'bg-slate-500' : 'bg-gray-300'} rounded-full px-1 duration-200" on:click="{toggle}">
    <div class="bg-white w-4 h-4 rounded-full shadow-md transform {checked && 'translate-x-4'} duration-200"></div>
</div>