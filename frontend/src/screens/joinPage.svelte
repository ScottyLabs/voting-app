<script lang="ts">
    let { toAdmin, toVoter } = $props();
    let sessionID = $state("");
</script>

<main>
    <div>
        <h1>CampusVoting</h1>
        <h3>Powered by ScottyLabs</h3>
    </div>

    <div></div>

    <div class="card">
        <input type="text" bind:value={sessionID} placeholder="Session ID" />
        <button onclick={toVoter} class="joinBtn"> JOIN SESSION </button>
    </div>

    <div class="row">
        <h3 class="session">Want to create a session?</h3>
        <button onclick={toAdmin} class="sessBtn"> Create Session </button>
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
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

    .card {
        width: "420";
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    .row {
        display: flex;
        gap: 1em;
    }

    .sessBtn {
        width: 100%;
        margin-top: none;
        background-color: transparent;
        color: var(--colors-primary);
        border: none;
        border-radius: 0px;
        font-size: 20px;
        padding: 0px 0px;
        cursor: pointer;
    }

    .joinBtn {
        margin-top: 0em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

    .joinBtn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    .session {
        margin-top: 1em;
        margin-bottom: 1em;
        color: black;
    }

    h1 {
        margin-top: 0.3em;
        margin-bottom: 0.3em;
        color: var(--colors-primary);
    }

    h3 {
        margin-top: 0.5em;
        margin-bottom: 2em;
        color: var(--colors-text);
        white-space: nowrap;
    }
</style>
