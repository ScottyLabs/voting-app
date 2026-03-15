interface Motion {
    num: number;
    description: string; // format: "YYYY-MM-DDTHH:MM"
    threshold: string; // this ^
    quorum: string;
    style: string;
    timer: Time;
}

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

interface Ballot {
    name: string;
    start_time: string; // format: "YYYY-MM-DD"
    end_time: string; // this ^
    vote_type: string;
}