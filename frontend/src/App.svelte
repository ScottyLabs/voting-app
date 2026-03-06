<script lang="ts">
    import Welcome from "./screens/welcome.svelte";
    import Welcome2 from "./screens/welcome2.svelte";
    import Voting from "./screens/voting.svelte";
    import VotingMotion from "./screens/votingMotion.svelte";
    import SessionCreation from "./screens/sessionCreation.svelte";
    import ResultsAdmin from "./screens/resultsAdmin.svelte";
    import ResultsVoter from "./screens/resultsVoter.svelte";

    let screen = "welcome";
</script>

{#if screen === "welcome"}
    <Welcome onNext={() => (screen = "welcome2")} />
{:else if screen === "welcome2"}
    <Welcome2
        toVoter={() => (screen = "voting")}
        toAdmin={() => (screen = "SessionCreation")}
    />
{:else if screen === "voting"}
    <Voting onNext={() => (screen = "votingMotion")} />
{:else if screen === "votingMotion"}
    <VotingMotion onNext={() => (screen = "ResultsVoter")} />
{:else if screen === "SessionCreation"}
    <SessionCreation onNext={() => (screen = "ResultsAdmin")} />
{:else if screen === "ResultsAdmin"}
    <ResultsAdmin onNext={() => (screen = "SessionCreation")} />
{:else if screen === "ResultsVoter"}
    <ResultsVoter onNext={() => (screen = "welcome2")} />
{/if}

<style>
</style>
