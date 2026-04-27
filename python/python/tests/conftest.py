import pytest
from unittest.mock import AsyncMock, patch


@pytest.fixture
def base_url():
    return "http://localhost:9000"


@pytest.fixture
def api_key():
    return "test-api-key-12345"


@pytest.fixture
def mock_httpx_client():
    """Provides a mocked httpx.AsyncClient for unit tests."""
    with patch("httpx.AsyncClient") as mock:
        yield mock
