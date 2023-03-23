from web3_rush.web3_rush import Web3
import pytest

class TestBenchEthBlockNumber:
    def org_block_number(self, web3_original):
        _ = web3_original.eth.block_number
    
    def rush_block_number(self, web3: Web3):
        _ = web3.eth.block_number

    @pytest.mark.parametrize(
        "case", ["web3", "original"]
    )
    def test_block_number(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org_block_number if case == "original" else self.rush_block_number
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)

class TestBenchEthChainId:    
    def org_chain_id(self, web3_original):
        _ = web3_original.eth.chain_id
    
    def rush_chain_id(self, web3: Web3):
        _ = web3.eth.chain_id
        
    @pytest.mark.parametrize(
        "case", ["web3", "original"]
    )
    def test_chain_id(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org_chain_id if case == "original" else self.rush_chain_id
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)

class TestBenchEthAccounts:
    def org_accounts(self, web3_original):
        _ = web3_original.eth.accounts

    def rush_accounts(self, web3: Web3):
        _ = web3.eth.accounts
        
    @pytest.mark.parametrize(
        "case", ["web3", "original"]
    )
    def test_accounts(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org_accounts if case == "original" else self.rush_accounts
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)

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
        
    @pytest.mark.parametrize(
        "case", ["web3", "original"]
    )
    def test_send_transaction(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org_send_transaction if case == "original" else self.rush_send_transaction
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)
