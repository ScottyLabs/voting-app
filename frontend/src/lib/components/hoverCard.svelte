<script>
    let { open = false, children } = $props();
</script>

{#if open}
    <div class="card">
        <div class="tail"></div>
        {@render children?.()}
    </div>
{/if}

<style>
    .card {
        position: absolute;
        top: 36px;
        width: 240px;
        left: 60%;
        transform: translateX(-50%);

        background: white;
        border: 1px solid #ccc;
        border-radius: 6px;
        padding: 6px 0px;

        white-space: nowrap;
        z-index: 100;
    }

    .tail {
        position: absolute;
        top: -6px;
        left: 50%;
        transform: translateX(-50%);

        width: 0;
        height: 0;

        border-left: 6px solid transparent;
        border-right: 6px solid transparent;
        border-bottom: 6px solid white;
    }
</style>
