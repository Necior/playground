index.html: src/Main.elm
	elm make src/Main.elm

.PHONY: format
format: src/Main.elm
	elm-format --yes src/Main.elm

