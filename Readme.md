# Shall we smalltalk about weather, fellow humans?

## Glossary

- InSight (Interior Exploration using Seismic Investigations, Geodesy and Heat Transport) - mission designed to study deep interior of planet Mars
- Sol: Mars solar day, 24 hours, 39 minutes, 35.244 seconds

## Overview

Using NASA's InSight API for collecting daily weather updates from Mars. API documentation is provided under `assets`.

## Infrastructure setup

<img src="./assets/infra.png?raw=true" />

- Lambda function, ran on a schedule 
- EC2 running AMI with baked MongoDB
- EBS where MongoDB keeps its data

## Why?

- Why not? 
- Was curious about non-standard lambda environments
- Wanted to check out Rust new mongo driver

## Docker

Unless `lambda` feature is enabled code does not use any of the lambda-specific functionality. Instead compiled binary is ran in a container on a cron job.
Repo includes two Dockerfiles: one for x86_64 and one for arm64 architectures, to build both run:
```sh
make TAG=my-special-repo/lambda-mars manifest
```

This is going to leverage debian:buster-slim base image and docker buildx functionality, to:
- build & tag `my-special-repo/lambda-mars:armv8` image
- build & tag `my-special-repo/lambda-mars:amd64` image
- create & push manifest file `my-special-repo/lambda-mars:latest`
