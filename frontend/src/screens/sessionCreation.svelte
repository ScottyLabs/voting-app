<script lang="ts">
    import LongTextInput from "./../lib/components/longTextInput.svelte";
    import SelectMenu from "./../lib/components/selectMenu.svelte";
    import Popup from "../lib/components/popup.svelte";
    import backArr from "../lib/images/back_arrow.png";
    import ArrayEditor from "../lib/components/arrayEditor.svelte";
    import TimeScroller from "../lib/components/timeScroller.svelte";
    import HoverCard from "../lib/components/hoverCard.svelte";
    let { onNext, onBack } = $props();
    interface User {
        user_id: number;
        name: string;
        created_time: string;
    }

    interface Election {
        title: string;
        candidates: string[];
        style: string;
        timer: Time;
    }

    interface Time {
        days: number;
        hours: number;
        mins: number;
        secs: number;
    }

    interface Motion {
        num: number;
        description: string; // format: "YYYY-MM-DDTHH:MM"
        threshold: string; // this ^
        quorum: string;
        style: string;
        timer: Time;
    }

    // Placeholder values until finalization of what this entails
    const voteTypeOptions = ["Type 1", "Type 2", "TenType"];

    function user_new(): User {
        return {
            user_id: 6767,
            name: "",
            created_time: "",
        };
    }

    function election_new(): Election {
        return {
            title: "",
            candidates: [],
            style: "",
            timer: {
                days: 0,
                hours: 0,
                mins: 0,
                secs: 0,
            },
        };
    }

    function motion_new(): Motion {
        return {
            num: 67,
            description: "",
            threshold: "",
            quorum: "",
            style: "",
            timer: {
                days: 0,
                hours: 0,
                mins: 0,
                secs: 0,
            },
        };
    }

    let time: Time = $state({
        days: 0,
        hours: 0,
        mins: 0,
        secs: 10,
    });

    let motion: Motion = $state(motion_new());

    let election: Election = $state(election_new());

    function goNext() {
        onNext?.();
    }

    let Users: User[] = $state([]);

    let user1: User = {
        user_id: 69,
        name: "Max Tentype",
        created_time: "6767-67-67",
    };
    let user2: User = {
        user_id: 420,
        name: "Yiyoung Liu",
        created_time: "4200-67-67",
    };
    let user3: User = {
        user_id: 67,
        name: "Anish Pallati",
        created_time: "7676-67-67",
    };
    Users.push(user1);
    Users.push(user2);
    Users.push(user3);

    let temp = 100;
    for (let i = 0; i < 30; i++) {
        let newUser: User = user_new();
        temp++;
        newUser.name = "M";
        newUser.user_id = temp;
        Users.push(newUser);
    }

    let meetingCode: string = "3CMU67";

    let electionStyleOptions: string[] = [
        "Plurality Election",
        "Ranked Choice Election",
    ];

    let voteStyleOptions: string[] = [
        "Standard Vote",
        "Recorded (roll-call) Vote",
        "Secret Vote",
    ];

    let creatingMotion = $state(false);
    let creatingElection = $state(false);
    let inspectingUser = $state<User | null>(null);
    let inspectingAllUsers = $state(false);
    let timerEnded = $state(false);

    let voteThresholds: string[] = ["Majority", "2/3", "3/4", "Unanimous"];

    function deleteUser(i: number) {
        Users.splice(i, 1);
    }

    function pushMotion() {
        motion = motion_new();
        creatingMotion = true;
    }

    function pushElection() {
        election = election_new();
        creatingElection = true;
    }

    function inspectAllUsers() {
        inspectingAllUsers = true;
    }

    function onPopupClose() {
        creatingElection = false;
        creatingMotion = false;
        inspectingAllUsers = false;
        timerEnded = false;
    }

    function inspectUser(user: User) {
        console.log("inspecting...");
        inspectingUser = user;
    }

    function clearInspect() {
        console.log("stopped inspecting");
        inspectingUser = null;
    }

    function endTimer() {
        timerEnded = true;
    }

    // TODO: after backend exists it can pass on live results to here
    function getResults() {
        return "TBD";
    }
</script>

<Popup title="Results:" open={timerEnded} onClose={onPopupClose}>
    <div>
        {getResults()}
    </div>
    <hr style="margin-bottom: 0;" />
    <h3 style="margin: 0;">Push Results?</h3>
    <div
        class="row"
        style="justify-content: center; gap: 1rem; align-items: center;"
    >
        <button
            type="button"
            onclick={goNext}
            class="btn"
            style="padding: 10px 50px; margin:0"
        >
            Yes
        </button>
        <button
            type="button"
            onclick={onPopupClose}
            class="btn"
            style="padding: 10px 50px; margin: 0"
        >
            No
        </button>
    </div>
</Popup>

<Popup
    title="Participants (Headcount: {Users.length})"
    open={inspectingAllUsers}
    onClose={onPopupClose}
>
    <div class="button-list">
        {#each Users as user, i}
            <div
                class="slot-wrapper"
                role="group"
                onmouseenter={() => inspectUser(user)}
                onmouseleave={clearInspect}
            >
                <button onclick={() => deleteUser(i)} class="slotDel">
                    {user.name?.charAt(0)}
                </button>
                <HoverCard open={inspectingUser?.user_id === user.user_id}>
                    <div class="col">
                        <div>Name: {user.name}</div>
                        <div>UserID: {user.user_id}</div>
                        <div>Time Created: {user.created_time}</div>
                    </div>
                </HoverCard>
            </div>
        {/each}
    </div>
</Popup>

<Popup
    title="Motion #{motion.num}"
    open={creatingMotion}
    onClose={onPopupClose}
>
    <form onsubmit={onPopupClose}>
        <LongTextInput
            title="Description:"
            value={motion.description}
            emptyPlaceholder="Input Description"
        ></LongTextInput>

        <label>
            <h3>Quorum:</h3>
            <input type="text" bind:value={motion.quorum} required />
        </label>

        <SelectMenu
            title="Threshold:"
            value={motion.threshold}
            options={voteThresholds}
        ></SelectMenu>

        <SelectMenu
            title="Voting Style:"
            value={motion.style}
            options={voteStyleOptions}
        ></SelectMenu>

        <TimeScroller value={motion.timer}></TimeScroller>

        <button type="submit" class="submitBtn">Push Motion</button>
    </form>
</Popup>

<Popup title={election.title} open={creatingElection} onClose={onPopupClose}>
    <form onsubmit={onPopupClose}>
        <label>
            <h3>Title:</h3>
            <input type="text" bind:value={election.title} required />
        </label>

        <ArrayEditor title="Candidates" items={election.candidates}
        ></ArrayEditor>

        <SelectMenu
            title="Election Style:"
            value={election.style}
            options={electionStyleOptions}
        ></SelectMenu>

        <TimeScroller value={election.timer}></TimeScroller>

        <button type="submit" class="submitBtn">Push Election</button>
    </form>
</Popup>

<main>
    <h1>Voting App</h1>
    <div class="card">
        <div class="row">
            <h1>Meeting Code:</h1>
            <h1 style="color:var(--colors-primary)">{meetingCode}</h1>
        </div>
        <hr />

        <div class="row">
            <h1>Participants</h1>
            <div class="container">
                <div class="button-list">
                    {#each Users.slice(0, 30 - 1) as user}
                        <div
                            class="slot-wrapper"
                            role="group"
                            onmouseenter={() => inspectUser(user)}
                            onmouseleave={clearInspect}
                        >
                            <button class="slot">
                                {user.name?.charAt(0)}
                            </button>
                            <HoverCard
                                open={inspectingUser?.user_id ===
                                    user.user_id && !inspectingAllUsers}
                            >
                                <div class="col">
                                    <div>Name: {user.name}</div>
                                    <div>UserID: {user.user_id}</div>
                                    <div>Time Created: {user.created_time}</div>
                                </div>
                            </HoverCard>
                        </div>
                    {/each}
                    {#if Users.length >= 30}
                        <button onclick={inspectAllUsers} class="slot plus"
                            >+</button
                        >
                    {/if}
                </div>
            </div>
        </div>
        <hr />
        <div class="row" style="margin-bottom: 0em">
            <button onclick={pushMotion} class="btn">Push a Motion</button>
            <button onclick={pushElection} class="btn">Push an Election</button>
        </div>
        <div class="row" style="marging-top=0em">
            <button onclick={endTimer} class="btn">END MEETING</button>
            <button class="btn" style="padding: 10px 175px">EXPORT</button>
        </div>
    </div>
    {#if !creatingElection && !creatingMotion && !inspectingAllUsers}
        <button onclick={onBack} class="backBtn">
            <img src={backArr} alt="Click me" />
        </button>
    {/if}
</main>

<style>
    .slot-wrapper {
        position: relative;
        width: 32px;
    }
    .btn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }
    .btn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }
    .card {
        width: fit-content;
        padding: 1.5rem;
        border-radius: 12px;
        background: #e0e0e0;
    }
    .container {
        border: 2px solid #ccc;
        padding: 8px;
        border-radius: 8px;
        width: fit-content;
        background: #f8f8f8;
        overflow: visible;
    }
    .row {
        display: flex;
        justify-content: flex-start;
        width: 100%;
        margin-top: 0.5em;
        gap: 1em;
        overflow: visible;
    }

    .button-list {
        margin-right: 0.75em;
        display: grid;
        grid-template-columns: repeat(10, 32px);
        grid-auto-rows: 20px;
        overflow: visible;
        gap: 1rem;
    }

    .slot {
        height: 28px;
        min-width: 28px;
        font-size: 0.8rem;
        border: 1px solid #aaa;
        border-radius: 4px;
        background: white;
        cursor: pointer;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .slot:hover {
        background: #eee;
    }

    .slotDel {
        height: 28px;
        min-width: 28px;
        font-size: 0.8rem;
        border: 1px solid #aaa;
        border-radius: 4px;
        background: white;
        cursor: pointer;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .slotDel:hover {
        background: #f44a4a;
    }

    .plus {
        font-weight: bold;
    }

    hr {
        width: 100%;
        border: none;
        border-top: 2px solid var(--colors-text);
        margin-top: 1em;
        margin-bottom: 1em;
    }

    h1 {
        color: var(--colors-text);
        margin-bottom: 0.5em;
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
        gap: 0rem;
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

    .col {
        display: flex;
        flex-direction: column;
        gap: 0.25em;
        text-align: left;
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
</style>
