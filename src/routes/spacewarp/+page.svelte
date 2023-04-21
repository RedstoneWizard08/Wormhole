<script lang="ts">
    import { goto } from "$app/navigation";
    import banner from "../../assets/background_banner.png";

    const doInstall = () => {
        goto("/install");
    };
</script>

<div class="spacewarp-container">
    <div class="container">
        <img src={banner} alt="Space Warp Logo" class="logo" />
    </div>

    <button type="button" class="action" on:click={doInstall}> Install SpaceWarp </button>
</div>

<style lang="scss">
    .spacewarp-container {
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;

        user-select: none;

        background-color: #1f2120;

        .container {
            display: flex;
            justify-content: center;
            align-items: center;
            height: 55vh;
        }

        .logo {
            width: 65%;
            border-radius: 8px;
            user-select: none;
        }

        .action {
            border: 2px solid #3c3c3e;
            border-radius: 4px;
            background-color: transparent;
            user-select: none;
            color: #dddfdc;
            font-family: "manteka", serif;

            width: 30%;
            padding: 1.5% 0;

            margin-top: 2%;
            margin-left: 35%;

            cursor: pointer;
            outline: none;
            transition: color 0.5s ease, background-color 0.5s ease;

            &:hover {
                background-color: #3c3c3e;
            }
        }
    }
</style>
