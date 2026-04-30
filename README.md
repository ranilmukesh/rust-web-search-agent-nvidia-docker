# Rig DuckDuckGo Search Agent (NVIDIA OpenAI compatible)

This is a minimal Rig agent that uses a DuckDuckGo search tool and talks to the
NVIDIA OpenAI-compatible endpoint.

## Requirements

- Docker Desktop (Windows)

## Quick start (Docker)

1. Copy .env.example to .env and set NVIDIA_API_KEY.
2. Build the image:

   docker compose build

3. Run the agent:

   docker compose run --rm rig-ddg-agent "What is Rig and how do I get started?"

To increase logging (PowerShell):

  $env:RUST_LOG="rig::completions=trace,reqwest=debug"; docker compose run --rm rig-ddg-agent "What is Rig and how do I get started?"

To increase logging (cmd):

  set RUST_LOG=rig::completions=trace,reqwest=debug
  docker compose run --rm rig-ddg-agent "What is Rig and how do I get started?"

## Local run (optional)

If you want to run without Docker:

NVIDIA_API_KEY=... RIG_OPENAI_BASE_URL=https://integrate.api.nvidia.com/v1 \
  RUST_LOG="rig::completions=trace,reqwest=debug" \
  cargo run -- "Explain Rig in one paragraph"

## Notes

- Rig uses the OpenAI-compatible /v1 base URL. The default in this project is:
  https://integrate.api.nvidia.com/v1
- The Rust agent uses the Chat Completions API to match the NVIDIA endpoint.
- The DuckDuckGo tool uses the public instant answer API:
  https://api.duckduckgo.com/?q=...&format=json

## Python example (OpenAI compatible)

See scripts/nvidia_openai_compat.py

## Shell example (OpenAI compatible)

See scripts/nvidia_openai_compat.sh

real logs from a local run in our setup:

#13 naming to docker.io/library/rust-rig-rig-ddg-agent:latest done
#13 unpacking to docker.io/library/rust-rig-rig-ddg-agent:latest 0.1s done
#13 DONE 0.6s

#14 resolving provenance for metadata file
#14 DONE 0.0s
[+] build 1/1
 ✔ Image rust-rig-rig-ddg-agent Built           121.6s
PS D:\rust-rig> ^C
PS D:\rust-rig> $env:RUST_LOG="rig::completions=trace,reqwest=debug"; docker compose run --rm rig-ddg-agent "ipl 2026 finals date"
[+]  1/1te 1/1
 ✔ Network rust-rig_default Created               0.0s
Container rust-rig-rig-ddg-agent-run-b45a183c9b77 Creating 
Container rust-rig-rig-ddg-agent-run-b45a183c9b77 Created 
2026-04-30T20:32:32.659375Z DEBUG chat{gen_ai.operation.name="chat" gen_ai.provider.name="openai" gen_ai.request.model="nvidia/nemotron-3-super-120b-a12b"}: starting new connection 'Some("integrate.api.nvidia.com")'
2026-04-30T20:32:42.344294Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:32:48.903134Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:33:01.897475Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:33:06.765387Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:33:09.236636Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:33:11.676597Z DEBUG starting new connection: https://api.duckduckgo.com/
The IPL 2026 final is scheduled for **31 May 2026**. The tournament is set to run from 28 March to 31 May 2026, with the final typically taking place on the last day of the season【https://en.wikipedia.org/wiki/2026_Indian_Premier_League】.
PS D:\rust-rig> 