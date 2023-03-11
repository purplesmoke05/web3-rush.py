from web3_rush import Web3
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
        _ = web3_original.eth.accounts

    def rush_accounts(self, web3: Web3):
        _ = web3.eth.accounts
        
    def test_org_accounts(self, benchmark, web3_original):
        ret = benchmark.pedantic(self.org_accounts, kwargs={"web3_original": web3_original}, rounds=100, iterations=10)
    
    def test_rush_accounts(self, benchmark, web3: Web3):
        ret = benchmark.pedantic(self.rush_accounts, kwargs={"web3":web3}, rounds=100, iterations=10)
