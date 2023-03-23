from web3 import Web3 as Web3Original, HTTPProvider
from web3_rush.web3_rush import Web3
import pytest


@pytest.fixture(scope="session")
def web3():
    yield Web3("http://0.0.0.0:8545")

@pytest.fixture(scope="session")
def web3_original():
    yield Web3Original(HTTPProvider(endpoint_uri="http://0.0.0.0:8545"))
