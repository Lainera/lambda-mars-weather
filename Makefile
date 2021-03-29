TAG="lambda_mars"

.PHONY: lambda, lambda-run, rpi_image, x86_image, manifest

lambda:
	docker run --rm \
		-e CARGO_FLAGS="--features lambda" \
		-v ${PWD}:/code \
		-v ${HOME}/.cargo/registry:/root/.cargo/registry:ro \
		-v ${HOME}/.cargo/git:/root/.cargo/git:ro \
		softprops/lambda-rust@sha256:55ba7a22a4d9ee4def3f72d5e0d830357979eec5773befa0d409cc64d4f90449
lambda-run:
	unzip -o target/lambda/release/bootstrap.zip -d /tmp/lambda && docker run -i \
		-e MONGODB_URI=$(MONGODB_URI) \
		--rm -v /tmp/lambda:/var/task:ro \
		--network $(MONGO_NETWORK) \
		lambci/lambda@sha256:bfc36a6b45e993d236a98e510083d55df1dc1634822c6cd1aef97526997a3b34 '{"key":"value"}'

rpi_image:
	docker buildx build . -t "$(TAG):armv8" -f rpi.Dockerfile --platform=linux/arm64/v8 --push

x86_image:
	docker buildx build . -t "$(TAG):amd64" -f Dockerfile --platform=linux/amd64 --push

manifest: rpi_image x86_image
	docker manifest create "$(TAG):latest" "$(TAG):armv8" "$(TAG):amd64" --amend && docker manifest push "$(TAG):latest"

