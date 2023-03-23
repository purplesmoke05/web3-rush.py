from web3_rush.web3_rush import Web3
import pytest


class TestEthModule:
    def test_accounts(self, web3: Web3):
        accounts = web3.eth.accounts
        assert len(accounts) == 10

        assert accounts[0] == "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
        assert accounts[1] == "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
        assert accounts[2] == "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"
        assert accounts[3] == "0x90F79bf6EB2c4f870365E785982E1f101E93b906"
        assert accounts[4] == "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65"
        assert accounts[5] == "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc"
        assert accounts[6] == "0x976EA74026E726554dB657fA54763abd0C3a0aa9"
        assert accounts[7] == "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955"
        assert accounts[8] == "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f"
        assert accounts[9] == "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720"

    def test_fee_history(self, web3: Web3):
        fee_history: FeeHistory = web3.eth.fee_history(0, 0, [])
        assert fee_history.base_fee_per_gas == []
        assert fee_history.gas_used_ratio == []
        assert fee_history.oldest_block == 0
        assert fee_history.reward == []

    def test_chain_id(self, web3: Web3):
        assert web3.eth.chain_id == 31337

    def test_gas_price(self, web3: Web3):
        assert type(web3.eth.gas_price) == int

    def test_block_number(self, web3: Web3):
        assert type(web3.eth.block_number) == int

    def test_send_transaction(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "gas": None,
                "gas_price": None,
                "value": 3,
                "data": None,
                "nonce": None,
                "chain_id": None
            }
        )
        assert type(tx) == str
