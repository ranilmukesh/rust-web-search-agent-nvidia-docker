#!/usr/bin/env bash
set -euo pipefail

invoke_url="${OPENAI_BASE_URL:-https://integrate.api.nvidia.com}/v1/chat/completions"

payload=$(cat <<'JSON'
{
  "model": "deepseek-ai/deepseek-v4-pro",
  "messages": [{"role":"user","content":"Hello from curl."}],
  "temperature": 1,
  "top_p": 0.95,
  "max_tokens": 1024,
  "chat_template_kwargs": {"thinking": false},
  "stream": true
}
JSON
)

curl -sS -N \
  --request POST \
  --url "$invoke_url" \
  --header "Authorization: Bearer ${NVIDIA_API_KEY:?}" \
  --header "Content-Type: application/json" \
  --data "$payload"
