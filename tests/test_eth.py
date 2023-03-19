from web3_rush import Web3
import pytest


class TestEthModule:
    def test_block_number(self, web3: Web3):
        assert len(web3.eth.accounts) == 10
        print(web3.eth.accounts[0])
        assert web3.eth.chain_id == 31337
        assert web3.eth.gas_price == 2000000000
        # assert web3.eth.fee_history(0, 0) == None
        # assert web3.eth.block_number == "0.1.0"

        web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "value": 12345,
                "gas": None,
                "gas_price": None,
                "data": None,
                "nonce": None,
                "chain_id": None
            }
        )