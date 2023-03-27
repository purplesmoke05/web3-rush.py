from web3_rush import Web3
from web3 import Web3 as Web3Original
import pytest


class TestWeb3Api:
    def org(self, web3_original: Web3Original):
        _ = web3_original.api

    def rush(self, web3: Web3):
        _ = web3.api

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_api(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3ToHex:
    def org(self, web3_original: Web3Original):
        _ = web3_original.to_hex(0x000F)
        _ = web3_original.to_hex(0)
        _ = web3_original.to_hex(hexstr="0x1")

    def rush(self, web3: Web3):
        _ = web3.to_hex(0x000F)
        _ = web3.to_hex(0)
        _ = web3.to_hex("0x1")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_to_hex(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3ToInt:
    def org(self, web3_original: Web3Original):
        _ = web3_original.to_int(0x000F)
        _ = web3_original.to_int(0)
        _ = web3_original.to_int(hexstr="000F")

    def rush(self, web3: Web3):
        _ = web3.to_int(0x000F)
        _ = web3.to_int(0)
        _ = web3.to_int(hexstr="000F")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_to_int(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3ToWei:
    def org(self, web3_original: Web3Original):
        _ = web3_original.to_wei(1, "ether")
        _ = web3_original.to_wei(10, "mether")
        _ = web3_original.to_wei(1.1, "ether")

    def rush(self, web3: Web3):
        _ = web3.to_wei(1, "ether")
        _ = web3.to_wei(10, "mether")
        _ = web3.to_wei(1.1, "ether")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_to_int(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3FromWei:
    def org(self, web3_original: Web3Original):
        _ = web3_original.from_wei(1000000000000000000, "ether")
        _ = web3_original.from_wei(1000000000000000000.1, "ether")

    def rush(self, web3: Web3):
        _ = web3.from_wei(1000000000000000000, "ether")
        _ = web3.from_wei(1000000000000000000.1, "ether")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_to_int(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3IsAddress:
    def org(self, web3_original: Web3Original):
        _ = web3_original.is_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3_original.is_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    def rush(self, web3: Web3):
        _ = web3.is_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3.is_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_is_address(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3IsChecksumAddress:
    def org(self, web3_original: Web3Original):
        _ = web3_original.is_checksum_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3_original.is_checksum_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    def rush(self, web3: Web3):
        _ = web3.is_checksum_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3.is_checksum_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_is_checksum_address(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestWeb3ToChecksumAddress:
    def org(self, web3_original: Web3Original):
        _ = web3_original.to_checksum_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3_original.to_checksum_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    def rush(self, web3: Web3):
        _ = web3.to_checksum_address("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")
        _ = web3.to_checksum_address("0xd3cda913deb6f67967b99d67acdfa1712c293601")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_to_checksum_address(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)
