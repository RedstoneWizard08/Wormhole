<script lang="ts">
import { goto } from "$app/navigation";
import Delete from "./Delete.svelte";
import { plugins } from "$api/stores";
import type { Instance } from "$bindings";

export let data: Instance;
// This is just the current instance in the parent for deletion (a reference)
export let current: Instance | null;
export let deleteing: boolean;

const clicked = () => {
    goto(`/${data.gameId}/instance/${data.id}`);
};

const doLaunch = async (e: MouseEvent) => {
    e.stopPropagation();

    // unwrap(await commands.launchGame(data.game_id, data, null));
};

const doDelete = async (e: MouseEvent) => {
    e.stopPropagation();

    if (deleteing) return;

    current = data;
    deleteing = true;
};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="instance-container" on:click={clicked}>
    <img
        src={$plugins.find((v) => v.game == data.gameId)?.banner_url}
        class="banner"
        alt={"background"} />

    <p class="name">{data.name}</p>

    <div class="buttons">
        <button type="button" class="__workaround__action launch" on:click={doLaunch}>
            <i class="icon fa-solid fa-play" />
        </button>

        <Delete action={doDelete} clazz="__workaround__action" />
    </div>
</div>

<style lang="scss">
    .instance-container {
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-direction: column;

        user-select: none;

        width: 100%;
        height: auto;

        margin: 5% 0;

        border-radius: 8px;
        background-color: var(--secondary-highlight-color);

        cursor: pointer;
        transition: background-color 0.5s ease;

        .buttons {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-end;

            width: 100%;
            height: 18%;

            opacity: 0;
            margin-bottom: 2%;

            transition: opacity 0.5s ease;

            .launch {
                background-color: #2c8c2c;
                border: none;
                color: white;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #2cac2c;
                }
            }
        }

        &:hover {
            background-color: var(--secondary-selected-color);

            .buttons {
                opacity: 1;
            }
        }

        .banner {
            width: 100%;
            margin-bottom: 8%;
            object-fit: contain;
            border-radius: 8px 8px 0 0;
        }

        .name {
            font-size: 12pt;
            margin: 0;
            text-align: center;
        }
    }
</style>
