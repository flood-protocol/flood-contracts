import json
import os

class FloodABIs:
    def __init__(self):
        this_dir, this_filename = os.path.split(__file__)
        abi_dir = os.path.join(this_dir, 'abis')
        for contract_abi in os.listdir(abi_dir):
            contract_name = contract_abi[:-5]
            contract_path = os.path.join(abi_dir, contract_abi)
            with open(contract_path, 'r') as f:
                contract = json.load(f)
                setattr(self, contract_name, contract['abi'])