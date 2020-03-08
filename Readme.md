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

