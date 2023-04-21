<script lang="ts">
    import { goto } from "$app/navigation";
    import { type InstanceInfo, KSPGame } from "../api/instance";
    import ksp1logo from "../assets/ksp.png";
    import ksp2logo from "../assets/ksp2.png";
    import { invoke_proxy } from "../api/invoke";

    export let data: InstanceInfo;
    export let instanceToDelete: InstanceInfo | null;
    export let deleteing: boolean;

    const clicked = () => {
        goto(`/${data.game}/instance/${data.id}`);
    };

    const doLaunch = (e: MouseEvent) => {
        e.stopPropagation();

        invoke_proxy("launch", {
            instanceId: data.id,
        });
    };

    const doDelete = (e: MouseEvent) => {
        e.stopPropagation();

        instanceToDelete = data;
        deleteing = !deleteing;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="instance-container" on:click={clicked}>
    {#if data.game == KSPGame.KSP2 || data.game.toString() == "KSP2"}
        <img src={ksp2logo} class="logo" alt={"background"} />
    {:else}
        <img src={ksp1logo} class="logo" alt={"background"} />
    {/if}

    <p class="name">{data.name}</p>

    <div class="buttons">
        <button type="button" class="action" id="launch-button" on:click={doLaunch}>
            <i class="icon fa-solid fa-play" />
        </button>

        <button type="button" class="action" id="delete-button" on:click={doDelete}>
            <i class="icon fa-solid fa-trash-can" />
        </button>
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

            button {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                width: 1.5rem;
                height: 1.5rem;

                border-radius: 50%;
                margin: 0 0.25rem;

                cursor: pointer;
                outline: none;
            }

            #launch-button {
                background-color: #2c8c2c;
                border: none;
                color: white;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #2cac2c;
                }
            }

            #delete-button {
                background-color: #ac2c2c;
                border: none;
                color: white;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #dc2c2c;
                }
            }
        }

        &:hover {
            background-color: #4f5150;

            .buttons {
                opacity: 1;
            }
        }

        .logo {
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
