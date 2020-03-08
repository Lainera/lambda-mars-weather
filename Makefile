lambda:
	docker run --rm \
		-v ${PWD}:/code \
		-v ${HOME}/.cargo/registry:/root/.cargo/registry:ro \
		-v ${HOME}/.cargo/git:/root/.cargo/git:ro \
		softprops/lambda-rust
lambda-run:
	unzip -o target/lambda/release/bootstrap.zip -d /tmp/lambda && docker run -i \
		-e MONGODB_URI=$(MONGODB_URI) \
		--rm -v /tmp/lambda:/var/task:ro \
		--network $(MONGO_NETWORK) \
		lambci/lambda:provided '{"key":"value"}'
