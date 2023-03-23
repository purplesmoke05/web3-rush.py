from web3_rush.web3_rush import Web3
import pytest

class TestBenchEthBlockNumber:
    def org_block_number(self, web3_original):
        _ = web3_original.eth.block_number
    
    def rush_block_number(self, web3: Web3):
        _ = web3.eth.block_number

    def test_rush_block_number(self, benchmark, web3: Web3):
        ret = benchmark.pedantic(self.rush_block_number, kwargs={"web3":web3}, rounds=100, iterations=10)

    def test_org_block_number(self, benchmark, web3_original):
        ret = benchmark.pedantic(self.org_block_number, kwargs={"web3_original": web3_original}, rounds=100, iterations=10)

class TestBenchEthChainId:    
    def org_chain_id(self, web3_original):
        _ = web3_original.eth.chain_id
    
    def rush_chain_id(self, web3: Web3):
        _ = web3.eth.chain_id
        
    def test_rush_chain_id(self, benchmark, web3: Web3):
        ret = benchmark.pedantic(self.rush_chain_id, kwargs={"web3":web3}, rounds=100, iterations=10)

    def test_org_chain_id(self, benchmark, web3_original):
        ret = benchmark.pedantic(self.org_chain_id, kwargs={"web3_original": web3_original}, rounds=100, iterations=10)

class TestBenchEthAccounts:
    def org_accounts(self, web3_original):
        a = web3_original.eth.accounts

    def rush_accounts(self, web3: Web3):
        _ = web3.eth.accounts
        
    def test_org_accounts(self, benchmark, web3_original):
        ret = benchmark.pedantic(self.org_accounts, kwargs={"web3_original": web3_original}, rounds=100, iterations=10)
    
    def test_rush_accounts(self, benchmark, web3: Web3):
        ret = benchmark.pedantic(self.rush_accounts, kwargs={"web3":web3}, rounds=100, iterations=10)

class TestBenchEthSendTransaction:
    def org_send_transaction(self, web3_original):
        tx = web3_original.eth.send_transaction(
            {
                "from": web3_original.eth.accounts[0],
                "to": web3_original.eth.accounts[1],
                "value": 3,
            }
        )

    def rush_send_transaction(self, web3: Web3):
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
        
    def test_org_send_transaction(self, benchmark, web3_original):
        ret = benchmark.pedantic(self.org_send_transaction, kwargs={"web3_original": web3_original}, rounds=100, iterations=10)
    
    def test_rush_send_transaction(self, benchmark, web3: Web3):
        ret = benchmark.pedantic(self.rush_send_transaction, kwargs={"web3":web3}, rounds=100, iterations=10)
