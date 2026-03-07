<script lang="ts">
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <div class="card">
        <h1>Results for Admin Screen</h1>
    </div>
    <button onclick={handleClick}> Continue </button>
</main>

<style>
    .card {
        width: 420px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }
</style>
