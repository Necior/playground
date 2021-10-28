import uuid


class KeyValueManager:
    def __init__(self, remove_f):
        self.remove_f = remove_f

    def remove(self):
        self.remove_f()


class Repository:
    def __init__(self):
        self.mapping = {}

    def store(self, value):
        key = uuid.uuid4().hex
        self.mapping[key] = value

        def remove():
            try:
                del self.mapping[key]
            except KeyError:
                pass

        return KeyValueManager(remove)

    def __repr__(self):
        return "Repository: " + repr(self.mapping)


if __name__ == "__main__":
    repo = Repository()
    kvm = repo.store("one")
    print(repo)
    repo.store("two")
    print(repo)
    kvm.remove()
    print(repo)
