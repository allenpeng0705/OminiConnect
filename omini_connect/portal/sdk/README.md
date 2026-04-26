# OminiConnect Python SDK

`pip install ominiconnect` — then:

```python
from ominiconnect import OminiConnect

client = OminiConnect(api_key="sk-...")
agents = client.agents.list()
tools = client.tools.list()
result = client.tools.execute("github_list_repos", owner="me")
```

## Structure

```
sdk/
├── python/          # pip installable package
│   ├── pyproject.toml
│   └── ominiconnect/
│       ├── __init__.py
│       ├── client.py      # OminiConnect client
│       ├── agents.py      # Agent management
│       └── tools.py       # Tool list/execute
└── js/              # npm package
    ├── package.json
    └── src/
        ├── index.ts       # OminiConnectClient
        ├── agents.ts
        └── tools.ts
```