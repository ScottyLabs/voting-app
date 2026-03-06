<script lang="ts">
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <button onclick={handleClick}> Continue </button>
</main>
