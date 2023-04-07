import time
from web3_rush import Web3
from web3 import Web3 as Web3Original
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

    def test_get_transaction(self, web3: Web3, web3_original: Web3Original):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        time.sleep(1)

        result1 = web3.eth.get_transaction(tx)
        result2 = web3_original.eth.get_transaction(tx)

        assert result1.chain_id == 31337
        assert result1.hash == result2["hash"].hex()
        assert result1.block_hash == result2["blockHash"].hex()
        assert result1.block_number == result2["blockNumber"]
        assert result1.from_ == web3.eth.accounts[0]
        assert result1.to == result2["to"]
        assert result1.gas == result2.gas
        assert result1.max_fee_per_gas == result2["maxFeePerGas"]
        assert result1.max_priority_fee_per_gas == result2["maxPriorityFeePerGas"]
        assert result1.nonce == result2.nonce
        assert result1.r == result2["r"].hex()
        assert result1.s == result2["s"].hex()
        assert result1.v == result2.v
        assert result1.value == result2.value
        assert result1.transaction_index == result2["transactionIndex"]

    def test_wait_for_transaction(self, web3: Web3, web3_original: Web3Original):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 1,
                # "nonce": 2,
                "gas": 30000,
                "gas_price": 4000000000,
                "data": "",
            }
        )
        assert type(tx) == str

        receipt1 = web3.eth.wait_for_transaction_receipt(tx, 5, 0.1)
        receipt2 = web3_original.eth.wait_for_transaction_receipt(tx, 5, 0.1)

        assert receipt1.block_hash == receipt2["blockHash"].hex()
        assert receipt1.block_number == receipt2["blockNumber"]
        assert receipt1.from_ == web3.eth.accounts[0]
        assert receipt1.to == receipt2["to"]
        assert receipt1.contract_address == receipt2["contractAddress"]
        assert receipt1.cumulative_gas_used == receipt2["cumulativeGasUsed"]
        assert receipt1.effective_gas_price == receipt2["effectiveGasPrice"]
        assert receipt1.gas_used == receipt2["gasUsed"]
        assert receipt1.root == None
        assert receipt1.status == receipt2["status"]
        assert receipt1.transaction_hash == receipt2["transactionHash"].hex()
        assert receipt1.transaction_index == receipt2["transactionIndex"]

        assert len(receipt1.logs) == len(receipt2["logs"])
        assert receipt1.logs_bloom == receipt2["logsBloom"].hex()

    def test_get_transaction_count(self, web3: Web3, web3_original: Web3Original):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        result1 = web3.eth.get_transaction_count(web3.eth.accounts[0])
        result2 = web3_original.eth.get_transaction_count(web3.eth.accounts[0])
        assert result1 == result2

    def test_estimate_gas(self, web3: Web3, web3_original: Web3Original):
        result1 = web3.eth.estimate_gas(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "value": 3,
            }
        )
        result2 = web3_original.eth.estimate_gas(
            {
                "from": web3.eth.accounts[0],
                "to": web3.eth.accounts[1],
                "value": 3,
            }
        )

        assert result1 == result2

    def test_get_raw_transaction(self, web3: Web3, web3_original: Web3Original):
        tx = web3.eth.send_transaction(
            {
                "from": web3.eth.accounts[0],
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        assert type(tx) == str

        time.sleep(1)

        result1 = web3.eth.get_raw_transaction(tx)
        # result2 = web3_original.eth.get_raw_transaction(tx)
        # assert result1 == result2

    def test_get_block(self, web3: Web3, web3_original: Web3Original):
        result1 = web3.eth.get_block(0)
        result2 = web3_original.eth.get_block(0)

        assert result1.hash == result2["hash"].hex()
        assert result1.parent_hash == result2["parentHash"].hex()
        assert result1.state_root == result2["stateRoot"].hex()
        assert result1.transactions_root == result2["transactionsRoot"].hex()
        # assert result1.receipts_root == result2["receiptRoot"].hex()
        assert result1.number == result2["number"]
        assert result1.gas_used == result2["gasUsed"]
        assert result1.gas_limit == result2["gasLimit"]
        assert result1.extra_data == result2["extraData"].hex()
        assert result1.timestamp == result2["timestamp"]
        assert result1.difficulty == result2["difficulty"]
        assert result1.total_difficulty == result2["totalDifficulty"]
        assert result1.seal_fields == result2["sealFields"]
        assert result1.uncles == result2["uncles"]
        assert result1.transactions == result2["transactions"]
        assert result1.size == result2["size"]
        assert result1.mix_hash == result2["mixHash"].hex()
        assert result1.nonce == result2["nonce"].hex()
        assert result1.base_fee_per_gas == result2["baseFeePerGas"]
        assert result1.other != None
        assert result1.logs_bloom == result2["logsBloom"].hex()

    def test_get_balance(self, web3: Web3, web3_original: Web3Original):
        result1 = web3.eth.get_balance(web3.eth.accounts[0])
        result2 = web3_original.eth.get_balance(web3.eth.accounts[0])

        assert result1 == result2

    def test_get_code(self, web3: Web3, web3_original: Web3Original):
        result1 = web3.eth.get_code(web3.eth.accounts[0])
        result2 = web3_original.eth.get_code(web3.eth.accounts[0])

        assert result1 == result2.hex()

    def test_get_logs(self, web3: Web3):
        result = web3.eth.get_logs({"block_from": 0, "block_to": web3.eth.block_number})
        assert type(result) == list

    def test_account_from_key(self, web3: Web3, web3_original: Web3Original):
        account1 = web3.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )
        account2 = web3_original.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )

        assert account1.address == account2.address

    def test_account_sign_transaction(self, web3: Web3, web3_original: Web3Original):
        account1 = web3.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )
        account2 = web3_original.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )

        tx1 = account1.sign_transaction(
            {
                "from": "0x5ce9454909639D2D17A3F753ce7d93fa0b9aB12E",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 1,
                "nonce": 2,
                "gas": 21000,
                "gasPrice": 40000000,
                "data": "0x",
                "chainId": 1,
            }
        )
        tx2 = account2.sign_transaction(
            {
                "from": "0x5ce9454909639D2D17A3F753ce7d93fa0b9aB12E",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 1,
                "nonce": 2,
                "gas": 21000,
                "gasPrice": 40000000,
                "data": "0x",
                "chainId": 1,
            }
        )

        assert tx1.raw_transaction == tx2.rawTransaction.hex()
        assert tx1.hash == tx2.hash.hex()
        assert tx1.r == tx2.r
        assert tx1.s == tx2.s
        assert tx1.v == tx2.v

    def test_account_sign_transaction(self, web3: Web3, web3_original: Web3Original):
        account1 = web3.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )
        account2 = web3_original.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )

        tx1 = account1.sign_message("test_message")
        # tx2 = account2.sign_message("test_message")
