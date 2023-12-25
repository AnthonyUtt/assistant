# assistant (cooler name TBD)

## [[WORK IN PROGRESS]]

This repo holds my attempts at creating an AI-powered virtual assitant
that could potentially run inside a homelab and be networked to from other
devices on the network. The goal is to have a local LLM living on the network
that would parse incoming messages and decide how to act.

### Structure

#### `<root>/`

The root directory of the repo contains a Rust package that will be able to
interact with the assistant process. The structure of this app is TBD, but the
goal is to have a rudimentary TUI for chatting with the assistant.

#### `assistant/`

This is the directory that houses the assistant background service, which is
configured with a Dockerfile to be run inside its container.

##### `src/actions/`

This directory contains any actions that are available to be initiated by the
LLM. The goal is to have the LLM respond in valid JSON, which is then parsed
to indicate a specific method (or set of methods) to be called.

##### `src/handlers/`

This directory contains the processes predicated by the receipt of a message
in any given topic. Topics can have many handlers, and handlers can cover
multiple topics. For the time being, the determination of which method(s) to
call is handled by `src/handlers/mod.rs#handle_message`.

#### `mqtt/`

This directory contains configuration files for the MQTT server that is run
as part of the Docker group.
