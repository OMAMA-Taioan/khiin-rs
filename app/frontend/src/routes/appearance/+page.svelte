<script lang="ts">
    import { _, locale } from "svelte-i18n" 
    
    function updateLanguage(event: Event) {
        const target = event.target as HTMLSelectElement;
        locale.set(target.value);
    }
</script>

<h1 class="text-3xl mb-3">{$_('page.appearance.title')}</h1>

<div class="mt-8 max-w-md">
    <div class="grid grid-cols-1 gap-6">
        <label class="block">
            <span class="text-gray-700">{$_('page.appearance.theme')}</span>
            <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
              <option>{$_('page.appearance.auto')}</option>
              <option>{$_('page.appearance.light')}</option>
              <option>{$_('page.appearance.dark')}</option>
            </select>
          </label>
          <label class="block">
              <span class="text-gray-700">{$_('page.appearance.language')}</span>
              <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50" on:change={updateLanguage}>
                <option value="en">English</option>
                <option value="zh-TW">漢羅</option>
                <option value="oan-TW">Lô-má-jī</option>
              </select>
            </label>
            <label class="block">
              <span class="text-gray-700">{$_('page.appearance.font-size')}</span>
              <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                  <option>{$_('page.appearance.xs')}</option>
                  <option>{$_('page.appearance.sm')}</option>
                  <option>{$_('page.appearance.md')}</option>
                  <option>{$_('page.appearance.lg')}</option>
                  <option>{$_('page.appearance.xl')}</option>
                </select>
            </label>
            <label class="block">
                  <span class="text-gray-700">{$_('page.appearance.font')}</span>
                  <select class="block w-full mt-1 rounded-md border-slate-300 shadow-sm focus:border-slate-300 focus:ring focus:ring-slate-200 focus:ring-opacity-50">
                      <!-- TODO: Local these fonts -->
                      <option>Arial</option>
                      <option>Helvetica</option>
                      <option>黑體</option>
                  </select>  
            </label>
    </div>
</div>