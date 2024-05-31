<script lang="ts">
export let clazz: string | undefined;
export let action: (event: MouseEvent) => any;
</script>

<button type="button" class="button {clazz || ''}" on:click={action}>
    <i class="icon fa-solid fa-trash-can" />
</button>

<style scoped lang="scss">
    .button {
        background-color: #ac2c2c;
        border: none;
        color: white;

        transition: background-color 0.5s ease;

        &:hover {
            background-color: #dc2c2c;
        }
    }
</style>
