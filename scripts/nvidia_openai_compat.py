import os

from openai import OpenAI

api_key = os.environ["NVIDIA_API_KEY"]
base_url = os.environ.get("OPENAI_BASE_URL", "https://integrate.api.nvidia.com/v1")
model = os.environ.get("RIG_MODEL", "deepseek-ai/deepseek-v4-pro")

client = OpenAI(api_key=api_key, base_url=base_url)

response = client.chat.completions.create(
    model=model,
    messages=[{"role": "user", "content": "Say hello from NVIDIA OpenAI compatibility."}],
    temperature=1,
    top_p=0.95,
    max_tokens=512,
)

print(response.choices[0].message.content)
