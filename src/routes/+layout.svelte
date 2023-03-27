<script lang="ts">
    import "../styles/global.scss";
    import Header from "../components/Header.svelte";
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import LoadingPage from "../components/LoadingPage.svelte";

    let loading = true;

    onMount(async () => {
        if (browser) {
            const setup = (await import("../api/setup")).setup;

            await setup();

            loading = false;
        }
    });
</script>

<svelte:head>
    <title>Wormhole</title>
</svelte:head>

{#if loading}
    <LoadingPage />
{:else}
    <div class="app-container">
        <Header />

        <div class="main">
            <slot />
        </div>
    </div>
{/if}

<style lang="scss">
    .app-container {
        width: 100%;
        height: 100%;

        color: white;
        font-family: "manteka", serif;

        margin: 0;
        padding: 0;

        background-color: #1f2120;
    }

    .app-container {
        .main {
            width: 100%;
            height: 100%;

            position: fixed;
            margin-top: calc(2rem + 2%);
        }
    }
</style>
