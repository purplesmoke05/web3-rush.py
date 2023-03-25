from web3_rush import Web3
from web3 import Web3 as Web3Original
import pytest


class TestNetVersion:
    def org(self, web3_original: Web3Original):
        _ = web3_original.net.version

    def rush(self, web3: Web3):
        _ = web3.net.version

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_version(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


# NOTE: anvil does not have peer_count method
class _TestNetPeerCount:
    def org(self, web3_original: Web3Original):
        _ = web3_original.net.peer_count

    def rush(self, web3: Web3):
        _ = web3.net.peer_count

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_peer_count(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestNetListening:
    def org(self, web3_original: Web3Original):
        _ = web3_original.net.listening

    def rush(self, web3: Web3):
        _ = web3.net.listening

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_listening(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)
