<script>
    let { title = "Popup", children, open = false, onClose } = $props();
</script>

{#if open}
    <div class="overlay">
        <div class="card">
            <button class="close" type="button" onclick={onClose}>X</button>
            <h2>{title}</h2>
            {@render children?.()}
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.4);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .card {
        position: relative;
        background: white;
        padding: 1.5rem;
        border-radius: 12px;
        max-width: 500px;
        width: 90%;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }

    h2 {
        margin: 0 0 1rem 0;
        color: var(--colors-primary);
    }

    .close {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        width: 32px;
        height: 32px;
        font-size: 1.2rem;
        border: none;
        background: none;
        cursor: pointer;
        display: flex;
        justify-content: center;
        align-items: center;
        color: var(--colors-primary);
    }

    .close:hover {
        color: color-mix(in srgb, var(--colors-primary), black 20%);
    }
</style>
