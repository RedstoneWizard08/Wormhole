<script lang="ts">
    export let to: string;
</script>

<a href={to}>
    <div class="return-container">
        <div class="return-arrow">
            <i class="fa-solid fa-long-arrow-left" />
        </div>
        <div class="return-circle" />
    </div>
</a>

<style scoped lang="scss">
    .return-container {
        position: relative;

        .return-arrow {
            position: absolute;
            user-select: none;
            top: 10px;
            left: 10px;
            width: 40px;
            height: 40px;
            display: flex;
            justify-content: center;
            align-items: center;
            border-radius: 50%;
            background-color: #008000;
            z-index: 2;
            cursor: pointer;

            transition: background-color 0.25s ease;

            &:hover {
                background-color: #00a000;
            }

            i {
                font-size: 24px;
                color: white;
            }
        }

        .return-circle {
            position: absolute;
            user-select: none;
            top: 10px;
            left: 10px;
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background-color: green;
            z-index: 1;
        }
    }
</style>
