<script>
    import { _ } from "svelte-i18n";
    import SidebarItem from "./SidebarItem.svelte";
</script>

<div class="w-56 flex-none" />
<nav
    class="
        fixed
        flex-none
        h-screen
        w-56
        flex
        flex-col
        overflow-hidden
        rounded-tl-lg
        bg-slate-100
        "
>
    <h1 class="flex pl-4 pt-5 pb-1 text-xl">Khíín</h1>
    <ul class="flex flex-col flex-1">
        <li>
            <SidebarItem
                route="/"
                iconClass="bx-home"
                label=""
            />
        </li>
        <li>
            <SidebarItem
                route="/appearance"
                iconClass="bx-font-family"
                label={$_("global.nav.appearance")}
            />
        </li>
        <li>
            <SidebarItem
                route="/input"
                iconClass="bx-edit"
                label={$_("global.nav.input-settings")}
            />
        </li>
        <li>
            <SidebarItem
                route="/dictionary"
                iconClass="bx-book-open"
                label={$_("global.nav.dictionary")}
            />
        </li>
        <li class="mt-auto">
            <SidebarItem
                target="_blank"
                route="https://github.com/OMAMA-Taioan/khiin-rs"
                iconClass="bxl-github"
                label={$_("global.nav.github")}
            />
        </li>
    </ul>
</nav>
