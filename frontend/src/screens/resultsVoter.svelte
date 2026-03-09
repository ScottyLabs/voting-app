<script lang="ts">
    let { onNext } = $props();

    // Percent values
    let percentAbst: number = 10;
    let percentYay: number = 60;
    let percentNay: number = 100 - percentYay - percentAbst;

    let barColor = $derived("#ffa500");
    let message = $derived(" resulted in a tie");

    if (percentNay < percentYay) {
        barColor = "#3fb991";
        message = " is passed";
    } else if (percentNay > percentYay) {
        barColor = "#ff7563";
        message = " is rejected";
    }

    let motion: string = $state("Motion to touch Max Tentype Wen");

    function handleClick() {
        onNext();
    }

    const bars = [
        { label: "Pass", percent: percentYay, color: "#3fb991" },
        { label: "Reject", percent: percentNay, color: "#ff7563" },
        { label: "Abstain", percent: percentAbst, color: "#ffa500" },
    ];
</script>

<main>
    <div class="card">
        <div class="topBar" style="background-color: {barColor}"></div>
        <h2>Motion Results</h2>
        <hr />

        {#each bars as bar}
            <div class="resultRow">
                <span class="label">{bar.label}:</span>
                <div class="progress">
                    <div
                        class="fill"
                        style="width: {bar.percent}%; background: {bar.color}"
                    >
                        <span class="inside">{bar.percent}%</span>
                    </div>
                </div>
            </div>
        {/each}

        <blockquote class="quote" style="border-left: 4px solid {barColor}">
            {motion}{message}
        </blockquote>

        <hr />

        <div class="row">
            <div class="col">
                <div>Total Votes: ##</div>
                <div>Vote Closed: ## minutes ago</div>
            </div>
            <button onclick={handleClick} class="btn">BACK TO HOME</button>
        </div>
    </div>
</main>

<style>
    .topBar {
        width: 100%;
        height: 6px;
        background: var(--colors-secondary);
        border-radius: 6px 6px 0 0;
        position: absolute;
        top: 0;
        left: 0;
    }
    .card {
        width: 420px;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
        position: relative;
    }

    h2 {
        text-align: left;
        margin: 0;
        color: var(--colors-text);
    }

    hr {
        width: 100%;
        border: none;
        border-top: 1px solid black;
        margin: 0.5em 0;
    }

    .quote {
        align-self: stretch;
        text-align: left;
        border-left: 4px solid var(--colors-secondary);
        padding-left: 12px;
        margin: 0rem 0;
        color: #555;
        font-style: italic;
    }

    .resultRow {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        width: 100%;
        text-align: left;
    }

    .label {
        width: 60px;
        font-weight: bold;
        text-align: left;
    }

    .progress {
        flex: 1;
        height: 24px;
        background: var(--colors-text);
        border-radius: 12px;
        overflow: hidden;
        position: relative;
    }

    .fill {
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding-right: 6px;
        font-weight: bold;
        color: white;
        transition: width 0.5s ease;
    }

    .inside {
        font-size: 14px;
    }

    .row {
        display: flex;
        justify-content: space-between;
        width: 100%;
        margin-top: 0em;
    }

    .col {
        display: flex;
        flex-direction: column;
        gap: 0.25em;
        text-align: left;
    }

    .btn {
        background-color: var(--colors-secondary);
        color: black;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        padding: 0.5em 1em;
    }

    .btn:hover {
        background-color: color-mix(
            in srgb,
            var(--colors-secondary),
            black 10%
        );
    }
</style>
