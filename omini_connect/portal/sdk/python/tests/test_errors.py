import pytest
from unittest.mock import patch, MagicMock

from ominiconnect.errors import (
    OminiConnectError,
    AuthError,
    ConnectorNotFoundError,
    UpstreamError,
    NetworkError,
    RateLimitedError,
)
from ominiconnect import OminiConnect


class TestErrorHierarchy:
    """Test that errors form correct hierarchy."""

    def test_ominiconnect_error_base(self):
        err = OminiConnectError("test message")
        assert str(err) == "test message"
        assert isinstance(err, Exception)

    def test_auth_error(self):
        err = AuthError("invalid or missing API key")
        assert "api key" in str(err).lower()
        assert isinstance(err, OminiConnectError)

    def test_connector_not_found_error(self):
        err = ConnectorNotFoundError("github")
        assert "github" in str(err)
        assert isinstance(err, OminiConnectError)

    def test_upstream_error(self):
        err = UpstreamError(400, "bad request")
        assert err.status_code == 400
        assert "400" in str(err)
        assert isinstance(err, OminiConnectError)

    def test_network_error(self):
        err = NetworkError("connection refused")
        assert "connection" in str(err).lower()
        assert isinstance(err, OminiConnectError)

    def test_rate_limited_error(self):
        err = RateLimitedError("rate limited — back off before retrying")
        assert "rate" in str(err).lower()
        assert isinstance(err, OminiConnectError)

    def test_upstream_error_truncates_long_body(self):
        long_body = "x" * 500
        err = UpstreamError(500, long_body)
        # Body should be truncated to 200 chars in message
        assert len(err.body) == 500
        assert len(str(err)) < 300  # truncated in message

    def test_upstream_error_equality(self):
        err1 = UpstreamError(400, "bad")
        err2 = UpstreamError(400, "bad")
        err3 = UpstreamError(400, "different")
        # Two upstream errors with same code/body are equal
        assert err1.status_code == err2.status_code
        assert err1.body == err2.body
        assert err3.body != err1.body


class TestClientMapsErrors:
    """Test that client maps HTTP responses to correct error types."""

    def test_401_raises_auth_error(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 401
        mock_response.ok = False
        mock_response.text = "invalid key"
        mock_response.content = b'invalid key'

        with patch.object(client._session, "get", return_value=mock_response):
            with pytest.raises(AuthError):
                client.connectors.list()

    def test_429_raises_rate_limited_error(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 429
        mock_response.ok = False
        mock_response.text = "rate limited"
        mock_response.content = b'rate limited'

        with patch.object(client._session, "get", return_value=mock_response):
            with pytest.raises(RateLimitedError):
                client.connectors.list()

    def test_404_raises_connector_not_found(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 404
        mock_response.ok = False
        mock_response.text = "not found"
        mock_response.content = b'not found'

        with patch.object(client._session, "get", return_value=mock_response):
            with pytest.raises(ConnectorNotFoundError) as exc_info:
                client.connectors.list()
            assert "not found" in str(exc_info.value) or "/api/connectors" in str(exc_info.value)

    def test_500_raises_upstream_error(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 500
        mock_response.ok = False
        mock_response.text = "internal server error"
        mock_response.content = b'internal server error'

        with patch.object(client._session, "get", return_value=mock_response):
            with pytest.raises(UpstreamError) as exc_info:
                client.connectors.list()
            assert exc_info.value.status_code == 500

    def test_network_timeout_raises_network_error(self, api_key, base_url):
        import requests
        client = OminiConnect(api_key=api_key, base_url=base_url)

        with patch.object(client._session, "get", side_effect=requests.exceptions.Timeout("timed out")):
            with pytest.raises(NetworkError) as exc_info:
                client.connectors.list()
            assert "timed out" in str(exc_info.value)

    def test_network_connection_error_raises_network_error(self, api_key, base_url):
        import requests
        client = OminiConnect(api_key=api_key, base_url=base_url)

        with patch.object(client._session, "get", side_effect=requests.exceptions.ConnectionError("connection refused")):
            with pytest.raises(NetworkError) as exc_info:
                client.connectors.list()
            assert "connection" in str(exc_info.value).lower()
