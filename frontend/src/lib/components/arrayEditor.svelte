<script lang="ts">
    let { title = "List", items = $bindable([]) } = $props();

    function addItem() {
        items.push("");
    }

    function removeItem(i: number) {
        items.splice(i, 1);
    }
</script>

<div class="container">
    <h3>{title}</h3>

    {#each items as item, i}
        <div class="item">
            <textarea
                bind:value={items[i]}
                required
                placeholder="Enter text here..."
                oninput={(e) => {
                    const el = e.target as HTMLTextAreaElement;
                    el.style.height = "auto";
                    el.style.height = el.scrollHeight + "px";
                }}
            >
            </textarea>
            <button type="button" class="delete" onclick={() => removeItem(i)}
                >X</button
            >
        </div>
    {/each}

    <button type="button" class="add" onclick={addItem}> + </button>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 100%;
    }

    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    .item {
        display: flex;
        align-items: stretch;
    }

    textarea {
        flex: 1;
        resize: none;
        padding: 0.5rem;
        border-radius: 6px;
        border: 1px solid #ccc;
    }

    .delete {
        margin-left: 0rem;
        width: 35px;
        display: flex;
        justify-content: center;
        align-items: center;
        border: none;
        background: #ff4d4d;
        color: white;
        border-radius: 6px;
        cursor: pointer;
    }

    .add {
        margin-top: 0.4rem;
        border: none;
        padding: 0.6rem;
        font-size: 18px;
        border-radius: 6px;
        cursor: pointer;
    }
</style>
