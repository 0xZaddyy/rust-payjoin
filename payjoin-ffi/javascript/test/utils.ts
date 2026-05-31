import { payjoin } from "payjoin";

class MemoryEventLog {
    readonly events: string[] = [];
    closed = false;

    save(event: string): void {
        this.events.push(event);
    }

    load(): string[] {
        return this.events;
    }

    close(): void {
        this.closed = true;
    }
}

class MemoryEventLogAsync {
    readonly events: string[] = [];
    closed = false;

    async save(event: string): Promise<void> {
        this.events.push(event);
    }

    async load(): Promise<string[]> {
        return this.events;
    }

    async close(): Promise<void> {
        this.closed = true;
    }
}

export class InMemoryReceiverPersister
    extends MemoryEventLog
    implements payjoin.JsonReceiverSessionPersister {}

export class InMemorySenderPersister
    extends MemoryEventLog
    implements payjoin.JsonSenderSessionPersister {}

export class InMemoryReceiverPersisterAsync
    extends MemoryEventLogAsync
    implements payjoin.JsonReceiverSessionPersisterAsync {}

export class InMemorySenderPersisterAsync
    extends MemoryEventLogAsync
    implements payjoin.JsonSenderSessionPersisterAsync {}
