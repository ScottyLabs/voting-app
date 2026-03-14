<script lang="ts">
    let { title = "Popup", value, options } = $props();
</script>

<main>
    <h3>{title}</h3>
    <select bind:value required>
        <option value="" disabled>Select one...</option>
        {#each options as option}
            <option value={option}>{option}</option>
        {/each}
    </select>
</main>

<style>
    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }
    select {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid #ccc;
        box-sizing: border-box;
        font-size: 20px;
        margin-bottom: 0em;
    }
</style>
