version := $(shell cat VERSION)

.PHONY:
publish-docker-image: app.py Dockerfile VERSION
	docker buildx build --platform linux/arm64,linux/amd64 -t necior/httpmongo:$(version) --push .

