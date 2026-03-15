<script lang="ts">
    let { value = $bindable() } = $props();
</script>

<h3>Time Limit:</h3>
<div class="timer">
    <div class="unit">
        <input type="number" min="0" bind:value={value.days} />
        <span>day</span>
    </div>

    <div class="unit">
        <input type="number" min="0" max="23" bind:value={value.hours} />
        <span>hour</span>
    </div>

    <div class="unit">
        <input type="number" min="0" max="59" bind:value={value.mins} />
        <span>minute</span>
    </div>

    <div class="unit">
        <input type="number" min="0" max="59" bind:value={value.secs} />
        <span>seconds</span>
    </div>
</div>

<style>
    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    .timer {
        display: flex;
        width: 100%;
        gap: 0.75rem;
    }

    .unit {
        display: flex;
        flex-direction: column;
        align-items: center;
        flex: 1;
    }

    input {
        width: 100%;
        height: 40px;
        text-align: center;
        font-size: 18px;
        border-radius: 6px;
        border: 1px solid #ccc;
    }

    span {
        margin-top: 4px;
        font-size: 14px;
    }
</style>
