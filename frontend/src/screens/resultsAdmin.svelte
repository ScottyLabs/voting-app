<script lang="ts">
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <h1>Results for Admin Screen</h1>
    <button onclick={handleClick}> Continue </button>
</main>
