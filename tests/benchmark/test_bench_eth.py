from web3_rush import Web3
import pytest


class TestEthBlockNumber:
    def org(self, web3_original):
        _ = web3_original.eth.block_number

    def rush(self, web3: Web3):
        _ = web3.eth.block_number

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_block_number(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthChainId:
    def org(self, web3_original):
        _ = web3_original.eth.chain_id

    def rush(self, web3: Web3):
        _ = web3.eth.chain_id

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_chain_id(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthSyncing:
    def org(self, web3_original):
        _ = web3_original.eth.syncing

    def rush(self, web3: Web3):
        _ = web3.eth.syncing

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_syncing(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthAccounts:
    def org(self, web3_original):
        _ = web3_original.eth.accounts

    def rush(self, web3: Web3):
        _ = web3.eth.accounts

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_accounts(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthSendTransaction:
    def org(self, web3_original):
        _ = web3_original.eth.send_transaction(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 3,
                # "chain_id": 31337
            }
        )

    def rush(self, web3: Web3):
        _ = web3.eth.send_transaction(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 3,
                "chain_id": 31337,
            }
        )

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_send_transaction(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthWaitForTransactionReceipt:
    def org(self, web3_original, tx: str):
        tx = web3_original.eth.wait_for_transaction_receipt(tx, 5, 0.1)

    def rush(self, web3: Web3, tx):
        tx = web3.eth.wait_for_transaction_receipt(tx, 5, 0.1)

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_wait_for_transaction_receipt(self, case, benchmark, web3_original, web3):
        if case == "rush":
            web3_mod = web3
        else:
            web3_mod = web3_original
        tx = web3_mod.eth.send_transaction(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )

        kwargs = (
            {"web3_original": web3_original, "tx": tx}
            if case == "original"
            else {"web3": web3, "tx": tx}
        )
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetTransaction:
    def org(self, web3_original, tx: str):
        tx = web3_original.eth.get_transaction(tx)

    def rush(self, web3: Web3, tx):
        tx = web3.eth.get_transaction(tx)

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_transaction(self, case, benchmark, web3_original, web3):
        if case == "rush":
            web3_mod = web3
        else:
            web3_mod = web3_original
        tx = web3_mod.eth.send_transaction(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )

        kwargs = (
            {"web3_original": web3_original, "tx": tx}
            if case == "original"
            else {"web3": web3, "tx": tx}
        )
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetTransactionCount:
    def org(self, web3_original, tx: str):
        _ = web3_original.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")

    def rush(self, web3: Web3, tx):
        _ = web3.eth.get_transaction_count("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_transaction_count(self, case, benchmark, web3_original, web3):
        if case == "rush":
            web3_mod = web3
        else:
            web3_mod = web3_original
        tx = web3_mod.eth.send_transaction(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
                "value": 3,
            }
        )
        kwargs = (
            {"web3_original": web3_original, "tx": tx}
            if case == "original"
            else {"web3": web3, "tx": tx}
        )
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetBlock:
    def org(self, web3_original):
        _ = web3_original.eth.get_block(0)

    def rush(self, web3: Web3):
        _ = web3.eth.get_block(0)

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_block(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetBalance:
    def org(self, web3_original):
        _ = web3_original.eth.get_balance("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")

    def rush(self, web3: Web3):
        _ = web3.eth.get_balance("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_balance(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetCode:
    def org(self, web3_original):
        _ = web3_original.eth.get_code("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")

    def rush(self, web3: Web3):
        _ = web3.eth.get_code("0xd3CdA913deB6f67967B99D67aCDFa1712C293601")

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_code(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthEstimateGas:
    def org(self, web3_original):
        _ = web3_original.eth.estimate_gas(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 3,
                # "chain_id": 31337
            }
        )

    def rush(self, web3: Web3):
        _ = web3.eth.estimate_gas(
            {
                "from": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "to": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "value": 3,
                "chain_id": 31337,
            }
        )

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_estimate_gas(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthGetLogs:
    def org(self, web3_original):
        _ = web3_original.eth.get_logs({"fromBlock": 0, "toBlock": web3_original.eth.block_number})

    def rush(self, web3: Web3):
        _ = web3.eth.get_logs({"block_from": 0, "block_to": web3.eth.block_number})

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_get_logs(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthAccountFromKey:
    def org(self, web3_original):
        _ = web3_original.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )

    def rush(self, web3: Web3):
        _ = web3.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_account_from_key(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)


class TestEthAccountSignTransaction:
    def org(self, account, web3_original):
        account = web3_original.eth.account.from_key(
            "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
        )
        _ = account.sign_transaction(
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

    def rush(self, account, web3):
        _ = account.sign_transaction(
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

    @pytest.mark.parametrize("case", ["rush", "original"])
    def test_account_sign_transaction(self, case, benchmark, web3_original, web3):
        kwargs = {"web3_original": web3_original} if case == "original" else {"web3": web3}
        if case == "original":
            kwargs["account"] = web3_original.eth.account.from_key(
                "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
            )
        else:
            kwargs["account"] = web3.eth.account.from_key(
                "0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364"
            )
        target_func = self.org if case == "original" else self.rush
        benchmark.pedantic(target_func, kwargs=kwargs, rounds=100, iterations=10)
