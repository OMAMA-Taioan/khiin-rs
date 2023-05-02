<script>
    import { _ } from 'svelte-i18n'
</script>
<nav class="flex h-full w-56 flex-col overflow-hidden rounded-tl-lg bg-slate-100">
    <h1 class="flex pl-4 pt-5 pb-1 text-xl">Khíín</h1>
    <ul class="flex flex-col">
        <li>
            <a href="/" class="flex h-12 transform flex-row items-center text-gray-500 transition-transform duration-200 ease-in hover:text-gray-800">
                <span class="inline-flex h-12 w-12 items-center justify-center text-lg"><i class="bx bx-home"></i></span>
                <span class="text-sm font-medium">{$_('global.nav.home')}</span>
            </a>
        </li>
        <li>
            <a href="/appearance" class="flex h-12 transform flex-row items-center text-gray-500 transition-transform duration-200 ease-in hover:text-gray-800">
                <span class="inline-flex h-12 w-12 items-center justify-center text-lg"><i class="bx bx-font-family"></i></span>
                <span class="text-sm font-medium">{$_('global.nav.appearance')}</span>
            </a>
        </li>
        <li>
            <a href="/input" class="flex h-12 transform flex-row items-center text-gray-500 transition-transform duration-200 ease-in hover:text-gray-800">
                <span class="inline-flex h-12 w-12 items-center justify-center text-lg"><i class="bx bx-edit"></i></span>
                <span class="text-sm font-medium">{$_('global.nav.input-settings')}</span>
            </a>
        </li>
        <li>
            <a href="/dictionary" class="flex h-12 transform flex-row items-center text-gray-500 transition-transform duration-200 ease-in hover:text-gray-800">
                <span class="inline-flex h-12 w-12 items-center justify-center text-lg"><i class="bx bx-book-open"></i></span>
                <span class="text-sm font-medium">{$_('global.nav.dictionary')}</span>
            </a>
        </li>
    </ul>
    <ul class="flex h-full justify-end flex-col">
        <li>
            <a href="https://github.com/aiongg/khiin-rs" target="_blank" class="flex h-12 transform flex-row items-center text-gray-500 transition-transform duration-200 ease-in hover:text-gray-800">
                <span class="inline-flex h-12 w-12 items-center justify-center text-lg"><i class="bx bxl-github"></i></span>
                <span class="text-sm font-medium">{$_('global.nav.github')}</span>
            </a>
        </li>
    </ul>
</nav>