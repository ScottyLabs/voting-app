<script lang="ts">
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<button onclick={handleClick}> Continue </button>
