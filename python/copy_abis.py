import json
import os
import shutil

contract_dirs = [d for d in os.listdir('../out') if d.endswith('.sol')]

for d in contract_dirs:
    contract_files = os.listdir(os.path.join('../out', d))
    for contract_file in contract_files:
        contract_name = contract_file[:-5]
        contract_path = os.path.join('../out', d, contract_file)

        shutil.copyfile(
            contract_path, 'flood_contract_abi/abis/' + contract_file)
