<script lang="ts">
    import { fly } from "svelte/transition";
    import backArr from "../lib/images/back_arrow.png";
    let { onNext, onBack } = $props();

    interface Ballot {
        name: string;
        start_time: string; // format: "YYYY-MM-DD"
        end_time: string; // this ^
        vote_type: string;
    }

    // Placeholder values until finalization of what this entails
    const voteTypeOptions = ["Type 1", "Type 2", "TenType"];

    let ballot: Ballot = {
        name: "",
        start_time: "",
        end_time: "",
        vote_type: "",
    };

    let message = $state("");
    let showBanner = $state(false);

    function createBallot(event: Event) {
        event.preventDefault();

        if (
            !ballot.name ||
            !ballot.start_time ||
            !ballot.end_time ||
            !ballot.vote_type
        ) {
            message = "Please fill out all fields";
            showBanner = true;
        } else {
            message = `Creating ballot: ${ballot.name} from ${ballot.start_time} to ${ballot.end_time}`;
            showBanner = true;

            onNext?.();
        }
        setTimeout(() => (showBanner = false), 3000);
    }
</script>

{#if showBanner}
    <div class="banner" in:fly={{ y: -30 }} out:fly={{ y: -30 }}>
        ${message}
    </div>
{/if}

<main>
    <h1>CampusVoting</h1>

    <div class="card">
        <h2>Configure Ballot</h2>
        <form onsubmit={createBallot}>
            <label>
                <h3>Name:</h3>
                <input
                    type="text"
                    bind:value={ballot.name}
                    placeholder="Enter ballot name"
                    required
                />
            </label>

            <label>
                <h3>Start Time:</h3>
                <input type="date" bind:value={ballot.start_time} required />
            </label>

            <label>
                <h3>End Time:</h3>
                <input type="date" bind:value={ballot.end_time} required />
            </label>

            <label>
                <h3>Voter Type:</h3>
                <select bind:value={ballot.vote_type} required>
                    <option value="" disabled>Select one...</option>
                    {#each voteTypeOptions as option}
                        <option value={option}>{option}</option>
                    {/each}
                </select>
            </label>

            <button type="submit" class="submitBtn">Create Ballot</button>
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

    input {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid #ccc;
        box-sizing: border-box;
        font-size: 20px;
        margin-bottom: 0em;
    }

    .banner {
        position: fixed;
        top: 20px;
        left: 50%;
        transform: translateX(-50%);

        background: var(--colors-secondary);
        color: var(--colors-text);

        padding: 12px 20px;
        border-radius: 8px;

        font-size: 16px;
        font-weight: 500;

        z-index: 1000;
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
