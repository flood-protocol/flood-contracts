import json
import os
from types import SimpleNamespace


class ABIS:
    def __init__(self):
        contract_dirs = [d for d in os.listdir('../../../out') if d.endswith('.sol')]

        for d in contract_dirs:
            contract_dir = os.listdir(os.path.join('../../../out', d))[0]
            contract_name = contract_dir[:-5]
            contract_path = os.path.join('../../../out', d, contract_dir)

            with open(contract_path, 'r') as f:
                contract = json.load(f)
                setattr(self, contract_name, contract['abi'])


abis = ABIS()
print(dir(abis))

