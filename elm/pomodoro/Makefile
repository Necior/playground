version := $(shell cat VERSION)

index.html: src/Main.elm
	elm make src/Main.elm

.PHONY:
format: src/Main.elm
	elm-format --yes src/Main.elm

.PHONY: develop
develop:
	elm reactor

.PHONY:
publish-docker-image: index.html Dockerfile VERSION
	docker buildx build --platform linux/arm64,linux/amd64 -t necior/elm-pomodoro:$(version) --push .

