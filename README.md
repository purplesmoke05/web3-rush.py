# Web3-rush.py 🚀

web3-rush.py is a Python library designed for fast and easy interaction with the Ethereum blockchain🌐. It provides an interface similar to web3.py while utilizing Rust internally, resulting in significantly faster performance⚡.

## Performance 📈

We have benchmarked the execution speed of web3-rush.py against web3.py. The results can be viewed on a separate page here(https://purplesmoke05.github.io/web3-rush.py/dev/bench/). According to the benchmarks, web3-rush.py is approximately 200% faster or more compared to web3.py.


## Key Features 🔑
- Creating and managing Ethereum accounts 👤
- Sending and receiving Ether 💸
- Deploying and executing smart contracts 📜
- Retrieving data and transaction information on the Ethereum blockchain 🔍
- Listening and filtering events 🔔

## Installation 💻
To install web3-rush.py, run the following command:

```sh
pip install web3-rush
```

## Usage 📚

The usage of web3-rush.py is very similar to web3.py. Here's a simple example:

```python
from web3_rush import Web3

# Connect to Ethereum node
web3 = Web3('https://your-ethereum-node-url')

# Get an account
account = web3.eth.accounts[0]

# Send Ether
transaction = {
    "from": account,
    "to": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
    "value": web3.toWei(1, "ether"),
    "gas": 21000,
    "gasPrice": web3.toWei("50", "gwei"),
    "nonce": web3.eth.get_transaction_count(account),
}

# Sign and send transaction
transaction_hash = web3.eth.send_transaction(transaction)

# Get transaction receipt
transaction_receipt = web3.eth.wait_for_transaction_receipt(transaction_hash)

print(transaction_receipt)
```

## Documentation 📖

For detailed documentation, please visit here(TBD).

## Contributing 🤝

Contributions to the project are highly appreciated! We welcome bug reports, feature suggestions, pull requests, and any other form of contribution.

## License ⚖️

web3-rush.py is released under the GNU General Public License.
