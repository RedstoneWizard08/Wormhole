<div class="loader">
    <div class="spinner" />
    <p>Loading...</p>
</div>

<style lang="scss">
    .loader {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background-color: var(--base-color);
        font-family: Ubuntu;

        user-select: none;
    }

    .spinner {
        border: 8px solid #eee;
        border-top: 8px solid #007bff;
        border-radius: 50%;
        width: 50px;
        height: 50px;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }

        100% {
            transform: rotate(360deg);
        }
    }
</style>
