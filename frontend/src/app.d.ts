interface Event {
    id: number;
    event_type: string;
    name: string;
    status: string;
    start_time: string;
    end_time: string | null;
    data: EventData;
    created_by_user_id: number;
    organization_id: number;
}

interface EventData {
    description: string;
    session_code: string;
    vote_type: "motion" | "election";
    threshold: number;        // float, not string
    visibility: {
        participants: "hidden_until_release" | "live";
    };
    proxy: boolean;
    vote_options: string[];
}

interface User {
    id: number;
    name: string;
    created_at: string;
}

interface Time {
    days: number;
    hours: number;
    mins: number;
    secs: number;
}