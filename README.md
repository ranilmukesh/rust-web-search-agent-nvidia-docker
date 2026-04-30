# Rig DuckDuckGo Agent
> **An async Rust-based agent leveraging the Rig framework, DuckDuckGo's instant answer API, and NVIDIA's OpenAI-compatible inference endpoints.**

This repository provides a minimal, high-performance CLI agent capable of querying the web via DuckDuckGo and synthesizing grounded responses using an LLM. It is built for seamless execution within containerized environments.

## 🛠 Architecture & Stack
* **Core:** Rust
* **Agent Framework:** `rig-core`
* **Network & Async:** `tokio` (multi-thread), `reqwest` (rustls-tls)
* **LLM Provider:** NVIDIA API (via `openai` compatibility layer)
* **Default Model:** `deepseek-ai/deepseek-v4-pro`

---

## 🚀 Getting Started

### 1. Environment Configuration
Clone the repository and duplicate the environment template:
```bash
cp .env.example .env
```
Update `.env` with your API credentials:
```env
NVIDIA_API_KEY=your_actual_api_key_here
RIG_OPENAI_BASE_URL=https://integrate.api.nvidia.com/v1
RIG_MODEL=deepseek-ai/deepseek-v4-pro
RUST_LOG=info
```

### 2. Execution via Docker (Recommended)
The application is fully containerized for cross-platform execution. 

**Build the image:**
```bash
docker compose build
```

**Run a query:**
```bash
docker compose run --rm rig-ddg-agent "What is Rig and how do I get started?"
```

### 3. Local Bare-Metal Execution
If you prefer running it directly via Cargo, ensure you have the latest Rust toolchain installed:

```bash
# Pass environment variables and the query directly
NVIDIA_API_KEY=your_key RIG_OPENAI_BASE_URL=https://integrate.api.nvidia.com/v1 \
RUST_LOG="rig::completions=trace,reqwest=debug" \
cargo run --release -- "Explain Rig in one paragraph"
```

---

## 🔍 Observability & Tracing

This project uses `tracing` and `tracing-subscriber` for granular execution logging. You can control the verbosity using the `RUST_LOG` environment variable.

**Enable Trace Logging (PowerShell):**
```powershell
$env:RUST_LOG="rig::completions=trace,reqwest=debug"; docker compose run --rm rig-ddg-agent "ipl 2026 finals date"
```

**Enable Trace Logging (Bash/CMD):**
```bash
export RUST_LOG="rig::completions=trace,reqwest=debug"
docker compose run --rm rig-ddg-agent "ipl 2026 finals date"
```

### Expected Execution Output
```text
[+]  1/1
 ✔ Network rust-rig_default Created               0.0s
Container rust-rig-rig-ddg-agent-run-b45a183c9b77 Creating 
Container rust-rig-rig-ddg-agent-run-b45a183c9b77 Created 
2026-04-30T20:32:32.659375Z DEBUG chat{gen_ai.operation.name="chat" gen_ai.provider.name="openai" gen_ai.request.model="nvidia/nemotron-3-super-120b-a12b"}: starting new connection 'Some("integrate.api.nvidia.com")'
2026-04-30T20:32:42.344294Z DEBUG starting new connection: https://api.duckduckgo.com/
2026-04-30T20:33:06.765387Z DEBUG starting new connection: https://api.duckduckgo.com/

The IPL 2026 final is scheduled for **31 May 2026**. The tournament is set to run from 28 March to 31 May 2026, with the final typically taking place on the last day of the season【https://en.wikipedia.org/wiki/2026_Indian_Premier_League】.
```

---

## 🐍 Alternative Implementations

The `scripts/` directory contains equivalent examples demonstrating how to interface with the NVIDIA OpenAI-compatible endpoint using alternative tools.

### Python
For Python environment isolation and dependency resolution, it is highly recommended to use `uv`.

```bash
# Run the Python compatibility script using uv
uv run --with openai scripts/nvidia_openai_compat.py
```

### Shell (cURL)
To test the endpoint directly via the terminal:
```bash
bash scripts/nvidia_openai_compat.sh
```