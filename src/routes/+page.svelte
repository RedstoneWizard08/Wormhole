<script>
    import { plugins } from "$api/stores";
</script>

<div class="home-container">
    <br />
    <br />
    <br />
    <h1 class="title">Welcome to Wormhole</h1>
    <h2 class="subtitle">Select a game!</h2>
    <br />
    <br />
    <br />
    <div class="links">
        {#each $plugins as plugin}
            <a class="link" href="/{plugin.game}">
                <img src={plugin.icon_url} alt={plugin.display_name} />
            </a>
        {/each}
    </div>
</div>

<style lang="scss">
    .home-container {
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;

        user-select: none;
        background-color: var(--base-color);

        .title {
            width: 100%;
            user-select: none;
            text-align: center;
        }

        .subtitle {
            width: 100%;
            user-select: none;
            text-align: center;
            font-size: 16pt;
            color: #dadada;
        }

        .links {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-around;

            .link {
                $size: 60px;

                padding: 1%;
                border-radius: 8px;
                background-color: var(--secondary-highlight-color);
                transition: background-color 0.25s ease;

                &:hover {
                    background-color: var(--secondary-selected-color);
                }

                img {
                    width: $size;
                    height: $size;
                }
            }
        }
    }
</style>
