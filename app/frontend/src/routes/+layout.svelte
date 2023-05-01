<script>
    import "beercss/dist/cdn/beer.min.css";
	import "./global.css"
    import Navbar from "../lib/Navbar.svelte";
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
