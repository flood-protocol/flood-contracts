import json
import os
import shutil

contract_dirs = [d for d in os.listdir('../out') if d.endswith('.sol')]

for d in contract_dirs:
    contract_dir = os.listdir(os.path.join('../out', d))[0]
    contract_name = contract_dir[:-5]
    contract_path = os.path.join('../out', d, contract_dir)

    shutil.copyfile(contract_path, 'src/flood-abi/abis/' + contract_dir)


