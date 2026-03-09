<script lang="ts">
    let { onNext } = $props();

    interface Ballot {
        name: string;
        start_time: string; // format: "YYYY-MM-DD"
        end_time: string; // this ^
        vote_type: string;
    }

    let ballot: Ballot = {
        name: "Ballot Name",
        start_time: "",
        end_time: "",
        vote_type: "",
    };

    let motion: string = $state("Motion to touch Max Tentype Wen");

    function vote(event: Event) {
        event.preventDefault();
        onNext?.();
    }

    function handleClick() {
        onNext();
    }
</script>

<main>
    <h1>CampusVoting</h1>
    <div class="card">
        <h2>Vote on Current Motion</h2>
        <hr />
        <blockquote class="quote">{motion}</blockquote>
        <form onsubmit={vote}>
            <label>
                <h3>Concerning this motion I vote...</h3>
                <select bind:value={ballot.vote_type} required>
                    <option value="" disabled>Select one...</option>
                    {#each ["Pass", "Reject", "Abstain"] as option}
                        <option value={option}>{option}</option>
                    {/each}
                </select>
            </label>
            <button type="submit" class="submitBtn">Submit Vote</button>
        </form>
    </div>
</main>

<style>
    h1 {
        color: white;
    }

    h2 {
        margin-top: 0em;
        margin-bottom: 0em;
        color: var(--colors-primary);
    }

    h3 {
        text-align: left;
        color: black;
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    form {
        margin-top: 0em;
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 1rem;
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

    hr {
        width: 100%;
        border: none;
        border-top: 1px solid #bdbdbd;
        margin: 0 0;
    }

    .quote {
        align-self: stretch;
        text-align: left;
        border-left: 4px solid var(--colors-primary);
        padding-left: 12px;
        margin: 1rem 0;
        color: #555;
        font-style: italic;
    }

    .submitBtn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

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
