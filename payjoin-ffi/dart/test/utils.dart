import "package:payjoin/payjoin.dart" as payjoin;

class _InMemoryPersister {
  final List<String> events = [];
  bool closed = false;

  void save(String event) {
    events.add(event);
  }

  List<String> load() {
    return events;
  }

  void close() {
    closed = true;
  }
}

class _InMemoryPersisterAsync {
  final List<String> events = [];
  bool closed = false;

  Future<void> save(String event) async {
    events.add(event);
  }

  Future<List<String>> load() async {
    return events;
  }

  Future<void> close() async {
    closed = true;
  }
}

class InMemoryReceiverPersister extends _InMemoryPersister
    implements payjoin.JsonReceiverSessionPersister {}

class InMemorySenderPersister extends _InMemoryPersister
    implements payjoin.JsonSenderSessionPersister {}

class InMemoryReceiverPersisterAsync extends _InMemoryPersisterAsync
    implements payjoin.JsonReceiverSessionPersisterAsync {}

class InMemorySenderPersisterAsync extends _InMemoryPersisterAsync
    implements payjoin.JsonSenderSessionPersisterAsync {}
