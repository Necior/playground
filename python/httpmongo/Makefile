.PHONY:
publish-dev-docker-image: app.py Dockerfile
	docker buildx build --platform linux/arm64,linux/amd64 -t necior/httpmongo:dev --push .

