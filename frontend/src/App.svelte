<script lang="ts">
    import WaitingPage from "./screens/waitingPage.svelte";
    import { slide } from "svelte/transition";
    import AuthPage from "./screens/authPage.svelte";
    import JoinPage from "./screens/joinPage.svelte";
    import VotingMotion from "./screens/votingMotion.svelte";
    import SessionCreation from "./screens/sessionCreation.svelte";
    import ResultsAdmin from "./screens/resultsAdmin.svelte";
    import ResultsVoter from "./screens/resultsVoter.svelte";

    let bgDark = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-backgroundDark")
        .trim();
    let bgLight = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-background")
        .trim();

    let screen = $state("auth");

    $effect(() => {
        if (screen === "auth") {
            document.body.style.backgroundColor = bgLight;
        } else if (screen === "join") {
            document.body.style.backgroundColor = bgLight;
        } else if (screen === "voting") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "votingMotion") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "SessionCreation") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "ResultsAdmin") {
            document.body.style.backgroundColor = bgLight;
        } else {
            document.body.style.backgroundColor = bgLight;
        }
    });
</script>

{#if screen === "auth"}
    <div transition:slide>
        <AuthPage onNext={() => (screen = "join")} />
    </div>
{:else if screen === "join"}
    <div transition:slide>
        <JoinPage
            toVoter={() => (screen = "waiting")}
            toAdmin={() => (screen = "SessionCreation")}
        />
    </div>
{:else if screen === "waiting"}
    <div transition:slide>
        <WaitingPage onNext={() => (screen = "votingMotion")} />
    </div>
{:else if screen === "votingMotion"}
    <div transition:slide>
        <VotingMotion onNext={() => (screen = "ResultsVoter")} />
    </div>
{:else if screen === "SessionCreation"}
    <div transition:slide>
        <SessionCreation
            onNext={() => (screen = "ResultsAdmin")}
            onBack={() => (screen = "join")}
        />
    </div>
{:else if screen === "ResultsAdmin"}
    <div transition:slide>
        <ResultsAdmin onNext={() => (screen = "SessionCreation")} />
    </div>
{:else if screen === "ResultsVoter"}
    <div transition:slide>
        <ResultsVoter onNext={() => (screen = "join")} />
    </div>
{/if}

<style>
    :global(body) {
        transition: background-color 0.6s ease-in-out;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
    }
</style>
