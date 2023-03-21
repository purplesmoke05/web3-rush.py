class Web3:
    eth: EthModule

class EthModule:
    fee_history: FeeHistory

class FeeHistory:
    base_fee_per_gas: list[int]
    gas_used_ratio: list[float]
    oldest_block: int
    reward: list[list[int]]
