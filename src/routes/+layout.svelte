<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import "../styles/global.scss";
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import LoadingPage from "../components/LoadingPage.svelte";
    import Sidebar from "../components/Sidebar.svelte";

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
        font-family: "manteka", serif;

        margin: 0;
        padding: 0;

        background-color: #1f2120;

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
