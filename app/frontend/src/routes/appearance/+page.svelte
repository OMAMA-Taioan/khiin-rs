<script lang="ts">
    import { settings, updateSettings } from "../store";

    const updateFontSize = (e: Event) => {
        const value = (e.target as HTMLInputElement).value;

        updateSettings({
            candidates: {
                font_size: parseInt(value),
            },
        });
    };

    const defaultFontSize = 16;
    const minFontSize = 12;
    const maxFontSize = 32;
    let fontSize: number;

    let sliderRightPct: number;

    $: {
        fontSize = $settings?.candidates?.font_size || defaultFontSize;
        sliderRightPct =
            ((maxFontSize - fontSize) / (maxFontSize - minFontSize)) * 100;
        console.log("Font size: ", fontSize);
        console.log("Slider %", sliderRightPct);
    }
</script>

<p>Hello</p>
<p>Current color: {$settings?.candidates?.color}</p>

<div class="middle align">
    <label class="slider large">
        <input
            type="range"
            min="12"
            max="32"
            value={fontSize}
            on:change={updateFontSize}
        />
        <span style="--pct:{sliderRightPct}%;" />
        <div class="tooltip">{fontSize}</div>
    </label>
</div>

<p>Current font size: {fontSize}</p>

<style>
    span {
        left: 0%;
        right: var(--pct);
    }
</style>
