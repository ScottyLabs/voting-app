<script lang="ts">
    import backArr from "../lib/images/back_arrow.png";
    let { onBack, onNext } = $props();

    let ballot: Ballot = {
        name: "Ballot Name",
        start_time: "",
        end_time: "",
        vote_type: "",
    };

    let motion: string = $state("");

    function createBallot(event: Event) {
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
        <h2>{ballot.name}</h2>
        <form onsubmit={createBallot}>
            <label>
                <h3>Proposal:</h3>
                <textarea
                    bind:value={motion}
                    oninput={(e) => {
                        const el = e.target as HTMLTextAreaElement;
                        el.style.height = "auto";
                        el.style.height = el.scrollHeight + "px";
                    }}
                >
                </textarea>
            </label>

            <button type="submit" class="submitBtn">Create Motion</button>
        </form>
    </div>
    <button onclick={onBack} class="backBtn">
        <img src={backArr} alt="Click me" />
    </button>
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
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    form {
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 1rem;
    }

    textarea {
        width: 100%;
        min-height: 25px;
        resize: vertical;
    }

    .backBtn {
        position: fixed;
        top: 20px;
        left: 20px;

        width: 40px;
        height: 40px;

        display: flex;
        justify-content: center;
        align-items: center;

        padding: 0;
        border: none;
        background: none;
    }

    .backBtn img {
        width: 24px;
        height: 24px;
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
