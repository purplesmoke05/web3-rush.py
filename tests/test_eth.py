import time
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

    def test_syncing(self, web3: Web3):
        result = web3.eth.syncing
        assert result == False

    def test_send_transaction(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "value": 3,
            }
        )
        assert type(tx) == str

    def test_get_transaction(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        time.sleep(1)

        result = web3.eth.get_transaction(tx)
        assert result.chain_id == 31337
        assert type(result.block_hash) == str
        assert type(result.block_number) == int
        assert result.from_ == web3.eth.accounts[0]
        assert result.to == None
        assert type(result.gas) == int
        assert result.hash == tx
        assert type(result.max_fee_per_gas) == int
        assert type(result.max_priority_fee_per_gas) == int
        assert type(result.nonce) == int
        assert type(result.r) == int
        assert type(result.s) == int
        assert type(result.v) == int
        assert type(result.value) == int
        assert type(result.transaction_index) == int

    def test_wait_for_transaction(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        receipt = web3.eth.wait_for_transaction_receipt(tx, 5, 0.1)
        assert type(receipt.block_hash) == str
        assert type(receipt.block_number) == int
        assert receipt.from_ == web3.eth.accounts[0]
        assert receipt.to == None
        assert type(receipt.contract_address) == str
        assert type(receipt.cumulative_gas_used) == int
        assert type(receipt.effective_gas_price) == int
        assert type(receipt.gas_used) == int
        assert receipt.root == None
        assert receipt.status == 1
        assert receipt.transaction_hash == tx
        assert type(receipt.transaction_index) == int
        assert receipt.status == 1

        assert len(receipt.logs) == 0
        assert type(receipt.logs_bloom) == str

    def test_get_transaction_count(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        result = web3.eth.get_transaction_count(web3.eth.accounts[0])
        assert type(result) == int

    def test_estimate_gas(self, web3: Web3):
        result = web3.eth.estimate_gas(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "value": 3,
            }
        )
        assert type(result) == int

    def test_get_raw_transaction(self, web3: Web3):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        time.sleep(1)

        result = web3.eth.get_raw_transaction(tx)
        assert type(result) == str

    def test_get_block(self, web3: Web3):
        result = web3.eth.get_block(0)

        assert type(result.hash) == str
        assert type(result.parent_hash) == str
        assert type(result.uncles_hash) == str
        assert type(result.author) == str
        assert type(result.state_root) == str
        assert type(result.transactions_root) == str
        assert type(result.receipts_root) == str
        assert type(result.number) == int
        assert type(result.gas_used) == int
        assert type(result.gas_limit) == int
        assert type(result.extra_data) == str
        assert type(result.timestamp) == int
        assert type(result.difficulty) == int
        assert type(result.total_difficulty) == int
        assert type(result.seal_fields) == list
        assert type(result.uncles) == list
        assert type(result.transactions) == list
        assert type(result.size) == int
        assert type(result.mix_hash) == str
        assert type(result.nonce) == str
        assert type(result.base_fee_per_gas) == int
        assert type(result.other) != None
        assert type(result.logs_bloom) == str

    def test_get_balance(self, web3: Web3):
        result = web3.eth.get_balance(web3.eth.accounts[0])

        assert type(result) == int

    def test_get_code(self, web3: Web3):
        result = web3.eth.get_code(web3.eth.accounts[0])

        assert result == "0x"

    def test_get_logs(self, web3: Web3):
        result = web3.eth.get_logs({"block_from": 0, "block_to": web3.eth.block_number})
        assert type(result) == list
