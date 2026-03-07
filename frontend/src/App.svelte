<script lang="ts">
    import { slide } from "svelte/transition";
    import Welcome from "./screens/welcome.svelte";
    import Welcome2 from "./screens/welcome2.svelte";
    import Voting from "./screens/voting.svelte";
    import VotingMotion from "./screens/votingMotion.svelte";
    import SessionCreation from "./screens/sessionCreation.svelte";
    import ResultsAdmin from "./screens/resultsAdmin.svelte";
    import ResultsVoter from "./screens/resultsVoter.svelte";

    let bgDark = getComputedStyle(document.documentElement).getPropertyValue(
        "--colors-backgroundDark",
    );
    let bgLight = getComputedStyle(document.documentElement).getPropertyValue(
        "--colors-background",
    );

    $: {
        if (screen === "welcome") {
            document.body.style.backgroundColor = bgLight;
        } else if (screen === "welcome2") {
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
    }

    let screen = "welcome";
</script>

{#if screen === "welcome"}
    <div transition:slide>
        <Welcome onNext={() => (screen = "welcome2")} />
    </div>
{:else if screen === "welcome2"}
    <div transition:slide>
        <Welcome2
            toVoter={() => (screen = "voting")}
            toAdmin={() => (screen = "SessionCreation")}
        />
    </div>
{:else if screen === "voting"}
    <div transition:slide>
        <Voting onNext={() => (screen = "votingMotion")} />
    </div>
{:else if screen === "votingMotion"}
    <div transition:slide>
        <VotingMotion onNext={() => (screen = "ResultsVoter")} />
    </div>
{:else if screen === "SessionCreation"}
    <div transition:slide>
        <SessionCreation
            onNext={() => (screen = "ResultsAdmin")}
            onBack={() => (screen = "welcome2")}
        />
    </div>
{:else if screen === "ResultsAdmin"}
    <div transition:slide>
        <ResultsAdmin onNext={() => (screen = "SessionCreation")} />
    </div>
{:else if screen === "ResultsVoter"}
    <div transition:slide>
        <ResultsVoter onNext={() => (screen = "welcome2")} />
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
