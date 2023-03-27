from web3_rush import Web3
import pytest


class TestWeb3Module:
    def test_api(self, web3: Web3):
        assert web3.api == "0.1.0"

    @pytest.mark.parametrize(
        "data, expect", [(0, "0x0"), (1, "0x1"), (0x0, "0x0"), (0x000F, "0xf"), (b"", "0x")]
    )
    def test_to_hex(self, data, expect, web3: Web3):
        assert web3.to_hex(data) == expect
        assert web3.toHex(data) == expect

    @pytest.mark.parametrize(
        "data, arg, expect",
        [
            (0, None, 0),
            (0x000F, None, 15),
            (b"\x00\x0F", None, 15),
            (b"\x00\x03\x0F", None, 783),
            (False, None, 0),
            (True, None, 1),
            ("0x000F", "hexstr", 15),
            ("000F", "hexstr", 15),
        ],
    )
    def test_to_int(self, data, arg, expect, web3: Web3):
        if arg is not None:
            d = {arg: data}
            assert web3.to_int(**d) == expect
            return
        assert web3.to_int(data) == expect

    @pytest.mark.parametrize(
        "number, unit, expect",
        [
            (1, "ether", 1000000000000000000),
        ],
    )
    def test_to_wei(self, number, unit, expect, web3: Web3):
        assert web3.to_wei(number, unit) == expect

    @pytest.mark.parametrize(
        "number, unit, expect",
        [
            (1000000000000000000, "ether", 1),
        ],
    )
    def test_from_wei(self, number, unit, expect, web3: Web3):
        assert web3.from_wei(number, unit) == expect

    def test_client_version(self, web3: Web3):
        assert web3.client_version == "anvil/v0.1.0"
