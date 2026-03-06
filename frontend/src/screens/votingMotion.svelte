<script lang="ts">
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <h1>Motion Voting Screen</h1>
    <button onclick={handleClick}> Continue </button>
</main>
