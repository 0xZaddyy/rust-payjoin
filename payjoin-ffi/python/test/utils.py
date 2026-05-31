import payjoin


class _InMemoryPersister:
    def __init__(self):
        self.events = []
        self.closed = False

    def save(self, event: str):
        self.events.append(event)

    def load(self):
        return self.events

    def close(self):
        self.closed = True


class _InMemoryPersisterAsync:
    def __init__(self):
        self.events = []
        self.closed = False

    async def save(self, event: str):
        self.events.append(event)

    async def load(self):
        return self.events

    async def close(self):
        self.closed = True


class InMemoryReceiverPersister(
    _InMemoryPersister, payjoin.JsonReceiverSessionPersister
):
    pass


class InMemorySenderPersister(_InMemoryPersister, payjoin.JsonSenderSessionPersister):
    pass


class InMemoryReceiverPersisterAsync(
    _InMemoryPersisterAsync, payjoin.JsonReceiverSessionPersisterAsync
):
    pass


class InMemorySenderPersisterAsync(
    _InMemoryPersisterAsync, payjoin.JsonSenderSessionPersisterAsync
):
    pass
