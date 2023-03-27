from .web3_rush import module

from .web3_rush import *  # noqa: F401

Web3 = module.Web3

from web3 import Web3 as Web3Original
from web3._utils.encoding import (
    to_hex,
)
from eth_utils import (
    from_wei,
    to_int,
    to_wei,
)


def create_class_method(method):
    @classmethod
    def method_wrapper(cls, *args, **kwargs):
        return method(*args, **kwargs)

    return method_wrapper


to_hex = create_class_method(to_hex)
to_int = create_class_method(to_int)
to_wei = create_class_method(to_wei)
from_wei = create_class_method(from_wei)

Web3.to_hex = to_hex
Web3.toHex = to_hex
Web3.to_int = to_int
Web3.toInt = to_int
Web3.to_wei = to_wei
Web3.toWei = to_wei
Web3.from_wei = from_wei
Web3.fromWei = from_wei
