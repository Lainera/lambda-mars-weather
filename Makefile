lambda:
	docker run --rm \
		-v ${PWD}:/code \
		-v ${HOME}/.cargo/registry:/root/.cargo/registry:ro \
		-v ${HOME}/.cargo/git:/root/.cargo/git:ro \
		softprops/lambda-rust@sha256:64b25595769255ab9f4d0549efe6b9c6346e436b11121852bcba1928a4fac4bc
lambda-run:
	unzip -o target/lambda/release/bootstrap.zip -d /tmp/lambda && docker run -i \
		-e MONGODB_URI=$(MONGODB_URI) \
		--rm -v /tmp/lambda:/var/task:ro \
		--network $(MONGO_NETWORK) \
		lambci/lambda@sha256:bfc36a6b45e993d236a98e510083d55df1dc1634822c6cd1aef97526997a3b34 '{"key":"value"}'
