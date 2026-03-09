<script lang="ts">
    import logo from "../lib/images/logoplaceholder.png";

    type Props = {
        onNext: () => void;
    };

    let { onNext }: Props = $props();

    function handleClick() {
        onNext();
    }
</script>

<main>
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
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
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
