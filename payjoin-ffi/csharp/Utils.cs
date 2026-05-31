namespace Payjoin.Tests;

internal abstract class MemoryEventLog
{
    private readonly List<string> _events = new();

    public bool Closed { get; private set; }

    public void Save(string @event)
    {
        _events.Add(@event);
    }

    public string[] Load()
    {
        return _events.ToArray();
    }

    public void Close()
    {
        Closed = true;
    }
}

internal abstract class MemoryEventLogAsync
{
    private readonly List<string> _events = new();

    public bool Closed { get; private set; }

    public Task Save(string @event)
    {
        _events.Add(@event);
        return Task.CompletedTask;
    }

    public Task<string[]> Load()
    {
        return Task.FromResult(_events.ToArray());
    }

    public Task Close()
    {
        Closed = true;
        return Task.CompletedTask;
    }
}

internal class InMemoryReceiverPersister : MemoryEventLog, JsonReceiverSessionPersister
{
}

internal class InMemorySenderPersister : MemoryEventLog, JsonSenderSessionPersister
{
}

internal class InMemoryReceiverPersisterAsync : MemoryEventLogAsync, JsonReceiverSessionPersisterAsync
{
}

internal class InMemorySenderPersisterAsync : MemoryEventLogAsync, JsonSenderSessionPersisterAsync
{
}
