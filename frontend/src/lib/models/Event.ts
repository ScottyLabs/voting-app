export class Event {
    id: number;
    event_type: string;
    name: string;
    status: string;
    start_time: string;
    end_time: string | null;
    data: EventData;
    created_by_user_id: number;
    organization_id: number;

    constructor(data: {
        id: number;
        event_type: string;
        name: string;
        status: string;
        start_time: string;
        end_time: string | null;
        data: EventData;
        created_by_user_id: number;
        organization_id: number;
    }) {
        this.id = data.id;
        this.event_type = data.event_type;
        this.name = data.name;
        this.status = data.status;
        this.start_time = data.start_time;
        this.end_time = data.end_time;
        this.data = data.data;
        this.created_by_user_id = data.created_by_user_id;
        this.organization_id = data.organization_id;
    }

    timeUntilEnd(): Time | null {
        if (!this.end_time) return null;
        const diff = new Date(this.end_time).getTime() - Date.now();
        if (diff <= 0) return { days: 0, hours: 0, mins: 0, secs: 0 };
        const secs = Math.floor(diff / 1000);
        return {
            days: Math.floor(secs / 86400),
            hours: Math.floor((secs % 86400) / 3600),
            mins: Math.floor((secs % 3600) / 60),
            secs: secs % 60,
        };
    }

    isExpired(): boolean {
        if (!this.end_time) return false;
        return new Date(this.end_time).getTime() < Date.now();
    }

    isMotion(): boolean {
        return this.data.vote_type === "motion";
    }

    isElection(): boolean {
        return this.data.vote_type === "election";
    }
}