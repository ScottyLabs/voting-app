export class User {
    id: number;
    name: string;
    created_at: string;

    constructor(data: {
        id: number;
        name: string;
        created_at: string;
    }) {
        this.id = data.id;
        this.name = data.name;
        this.created_at = data.created_at;
    }
}