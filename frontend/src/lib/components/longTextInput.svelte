<script lang="ts">
    let { title = "Popup", value, emptyPlaceholder } = $props();
</script>

<main>
    <h3>{title}</h3>
    <textarea
        bind:value
        required
        placeholder={emptyPlaceholder}
        oninput={(e) => {
            const el = e.target as HTMLTextAreaElement;
            el.style.height = "auto";
            el.style.height = el.scrollHeight + "px";
        }}
    >
    </textarea>
</main>

<style>
    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    textarea {
        width: 100%;
        min-height: 25px;
        resize: vertical;
    }
</style>
