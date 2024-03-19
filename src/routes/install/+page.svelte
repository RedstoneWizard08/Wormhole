<script lang="ts">
    import banner from "../../assets/background_banner.png";
    import { invoke_proxy } from "../../api/invoke";
    import { KSPGame } from "../../api/instance";
    import { listen_proxy } from "../../api/listen";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    let _listening = false;
    let installed = false;

    let status = {
        percent: "0%",
        message: "Waiting for start button...",
    };

    onMount(() => {
        if (!_listening) {
            listen_proxy("download_progress", (ev) => {
                const percent = (100 * ev.payload.received) / ev.payload.total;

                status = {
                    percent: `${percent}%`,
                    message: percent == 100 ? "Done!" : status.message,
                };
            });

            _listening = true;
        }
    });

    const getInstallDir = async (): Promise<Error | null> => {
        status = { percent: "0%", message: "Finding KSP2 location..." };

        const _dir = await invoke_proxy("get_install_dir", {
            gameId: KSPGame.KSP2,
        });

        if (/(:?[A-Z]:[\\/].+)|(:?\/.+)/gm.test(_dir)) {
            return null;
        }

        return new Error(`Could not find the KSP2 install directory! More details: ${_dir}`);
    };

    const doInstall = async (): Promise<Error | null> => {
        status = { percent: "50%", message: "Installing SpaceWarp..." };

        const res = await invoke_proxy("install_spacewarp", undefined);

        if (res == "Success") {
            return null;
        }

        return new Error(`Could not install SpaceWarp! More details: ${res}`);
    };

    const beginSetup = async () => {
        const err = await getInstallDir();

        if (err) {
            status = { ...status, message: err.message };
            return;
        }

        await doInstall();

        status = { percent: "100%", message: "Done!" };
        installed = true;
    };

    const goBack = () => {
        goto("/");
    };
</script>

<div class="install-progress-container">
    <img src={banner} alt="Space Warp Logo" class="logo" />

    <br />

    <h1 class="title">Installing SpaceWarp...</h1>

    <p class="progress">
        {status.percent} - {status.message}
    </p>

    <button class="action" id="start" on:click={!installed ? beginSetup : goBack}>
        {!installed ? "Start" : "Back"}
    </button>
</div>

<style lang="scss">
    .install-progress-container {
        width: 100%;
        height: 100%;

        margin: 0;
        padding: 0;

        background-color: #1f2120;

        .logo {
            width: 50%;
            user-select: none;
            margin: 4% 25%;
            border-radius: 8px;
        }

        .title {
            width: 100%;
            user-select: none;
            text-align: center;
        }

        .progress {
            width: 100%;
            margin: 4% 0;
            user-select: none;
            text-align: center;
        }

        .action {
            border: 2px solid #3c3c3e;
            border-radius: 4px;
            background-color: transparent;
            user-select: none;
            color: #dddfdc;
            font-family: "manteka", serif;

            width: 10%;
            padding: 1.5% 0;
            margin-left: 45%;

            cursor: pointer;
            outline: none;
            transition:
                color 0.5s ease,
                background-color 0.5s ease;

            &:hover {
                background-color: #3c3c3e;
            }
        }
    }
</style>
