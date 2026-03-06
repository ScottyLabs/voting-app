<script lang="ts">
    import logo from "/Users/johncao/Documents/GitHub/voting-app/frontend/src/lib/images/logoplaceholder.png";
    let { onNext } = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
    <div class="card">
        <div>
            <h1>CampusVoting</h1>
            <h3>Powered by ScottyLabs</h3>
        </div>
        <img
            src={logo}
            alt="CampusVoting Logo"
            class="logo"
            width="350px"
            height="350px"
        />
        <button onclick={handleClick} class="authBtn">
            SIGN IN WITH CMU SSO
        </button>
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .card {
        border: 5px solid var(--colors-background);
        border-radius: 12px;
        padding: 3rem;
        width: 450px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
    }

    .authBtn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 20px;
        padding: 10px 35px;
        cursor: pointer;
    }

    .authBtn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    h1 {
        margin-top: 0.3em;
        margin-bottom: 0.3em;
        color: var(--colors-primary);
    }

    h3 {
        margin-top: 0.5em;
        color: var(--colors-text);
    }
</style>
