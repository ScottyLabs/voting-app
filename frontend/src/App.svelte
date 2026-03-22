<script lang="ts">
    import WaitingPage from "./screens/waitingPage.svelte";
    import { slide } from "svelte/transition";
    import AuthPage from "./screens/authPage.svelte";
    import JoinPage from "./screens/joinPage.svelte";
    import VotingMotion from "./screens/votingMotion.svelte";
    import SessionCreation from "./screens/sessionCreation.svelte";
    import ResultsAdmin from "./screens/resultsAdmin.svelte";
    import ResultsVoter from "./screens/resultsVoter.svelte";
    import { Event } from "./lib/models/Event";
    import { User } from "./lib/models/User";

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    let screen = $state("auth");
    let currentUser = $state<User | null>(null);
    let currentEvent = $state<Event | null>(null);

    const fetchAttendance = async (sessionCode: string) => {
        const response = await fetch(
            `${API_BASE}/api/attendance/${sessionCode}`,
            { cache: "no-store" },
        );
        if (!response.ok) {
            throw new Error(`Failed to join event: ${response.status}`);
        }
        return await response.json();
    };

    let joinError = $state<string | null>(null);
    let joining = $state(false);

    const joinEvent = async (sessionCode: string) => {
        joining = true;
        joinError = null;
        try {
            const data = await fetchAttendance(sessionCode);
            // valid session code — navigate to waiting
            screen = "waiting";
        } catch (error) {
            joinError = "Invalid session code. Please try again.";
        } finally {
            joining = false;
        }
    };

    let bgDark = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-backgroundDark")
        .trim();
    let bgLight = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-background")
        .trim();

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
            {joinEvent}
            {joinError}
            {joining}
            toVoter={() => (screen = "waiting")}
            toAdmin={() => (screen = "SessionCreation")}
        />
    </div>
{:else if screen === "waiting"}
    <div transition:slide>
        <WaitingPage
            event={currentEvent}
            onNext={() => (screen = "votingMotion")}
        />
    </div>
{:else if screen === "votingMotion"}
    <div transition:slide>
        <VotingMotion
            event={currentEvent}
            user={currentUser}
            onNext={() => (screen = "ResultsVoter")}
        />
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
        <ResultsVoter
            event={currentEvent}
            user={currentUser}
            onNext={() => (screen = "join")}
        />
    </div>
{/if}
