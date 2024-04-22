<script lang="ts">
    import { goto } from "$app/navigation";
    import ksp1logo from "../assets/ksp.png";
    import ksp2logo from "../assets/ksp2.png";
    import Delete from "./Delete.svelte";
    import { plugins } from "../api/stores";

    export let data: InstanceInfo;
    // This is just the current instance in the parent
    export let current: InstanceInfo | null;
    export let deleteing: boolean;

    const clicked = () => {
        goto(`/${data.game}/instance/${data.id}`);
    };

    const doLaunch = (e: MouseEvent) => {
        e.stopPropagation();

        // TODO: Launch
    };

    const doDelete = (e: MouseEvent) => {
        e.stopPropagation();

        current = data;
        deleteing = !deleteing;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="instance-container" on:click={clicked}>
    <img src={$plugins[data.game].banner_url} class="banner" alt={"background"} />

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
        background-color: #3f4140;

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
            background-color: #4f5150;

            .buttons {
                opacity: 1;
            }
        }

        .banner {
            width: 100%;
            height: 50%;
            margin: 5% 0;
            object-fit: contain;
        }

        .name {
            font-size: 12pt;
            margin: 2% 0;
            text-align: center;
        }
    }
</style>
