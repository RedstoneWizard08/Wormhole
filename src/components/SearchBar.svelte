<script lang="ts">
    export let onSearch: (query: string) => void;

    const handleInput = (event: Event) => {
        const target = event.target as HTMLInputElement;

        onSearch(target.value);
    };
</script>

<div class="searchbar">
    <input type="text" placeholder="Search" on:input={handleInput} />
</div>

<style lang="scss">
    .searchbar {
        width: 100%;
        height: 100%;

        user-select: none;

        input {
            width: 100%;
            height: 100%;

            border: none;
            font-family: "manteka", serif;
            text-align: center;
            background-color: transparent;
            color: #ffffff;
            outline: none;
        }
    }
</style>
