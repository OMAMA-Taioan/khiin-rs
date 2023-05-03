<script lang="ts">
    export let route: string;
    export let iconClass: string;
    export let label: string;
    export let target: string = "";
</script>

<a
    href={route}
    target={target || null}
    class="
        flex
        h-12
        transform
        flex-row
        items-center
        text-gray-500
        transition-transform
        duration-200
        ease-in
        hover:text-gray-800
        "
>
    <span class="inline-flex h-12 w-12 items-center justify-center text-lg">
        <i class="bx {iconClass}" />
    </span>
    <span class="text-sm font-medium">{label}</span>
</a>
