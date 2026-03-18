<script>
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <h1>Waiting for Host to Start the Session...</h1>
    <div
        class="spinner"
        style="width: 40px; height: 40px; border-top-color: #FF3B3F;"
    ></div>
    <button onclick={onNext} class="btn"> Go to Motion </button>
</main>

<style>
    .btn {
        margin-top: 0em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

    .btn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }
    .spinner {
        border: 3px solid #ddd;
        border-radius: 50%;
        border-top-color: #555;
        animation: spin 1s linear infinite;
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
