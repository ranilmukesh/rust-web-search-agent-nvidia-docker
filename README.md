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

To increase logging:

  RUST_LOG=debug docker compose run --rm rig-ddg-agent "What is Rig and how do I get started?"

## Local run (optional)

If you want to run without Docker:

NVIDIA_API_KEY=... RIG_OPENAI_BASE_URL=https://integrate.api.nvidia.com/v1 \
  RUST_LOG=debug \
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
