<script lang="ts">
    import "$styles/global.scss";
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import LoadingPage from "$components/LoadingPage.svelte";
    import Sidebar from "$components/Sidebar.svelte";

    let loading = true;

    onMount(async () => {
        if (browser) {
            await (await import("$api/setup")).setup();

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
        <!-- <Header /> -->
        <Sidebar />

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
        font-family: "Ubuntu", serif;

        margin: 0;
        padding: 0;

        background-color: var(--base-color);

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
    }

    .app-container {
        .main {
            width: calc(98% - 2rem);
            height: 100%;

            position: fixed;
            margin-left: calc(2rem + 2%);
        }
    }
</style>
