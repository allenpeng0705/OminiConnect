import base64
import requests
import os

url = "https://api.minimaxi.com/v1/image_generation"
api_key = os.environ.get("MINIMAX_API_KEY")
headers = {"Authorization": f"Bearer {api_key}"}

payload = {
    "model": "image-01",
    "prompt": "A modern tech logo for 'OminiConnect', a universal API gateway. Design: interconnected circles or nodes forming a gateway/portal symbol, representing connection between AI agents and external APIs. Color scheme: deep blue (#1976d2) and teal (#00ACC1). Professional, clean, minimalist. No text. Square format.",
    "aspect_ratio": "1:1",
    "response_format": "base64",
}

response = requests.post(url, headers=headers, json=payload)
response.raise_for_status()
images = response.json()["data"]["image_base64"]

for i in range(len(images)):
    with open(f"omniconnect_logo_{i}.png", "wb") as f:
        f.write(base64.b64decode(images[i]))

print("Logo generated!")
