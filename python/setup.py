from setuptools import setup, find_packages
from os import environ

setup(
    name='flood_contract_abi',
    version="1.0.2b1",
    license='MIT',
    author="Nick Erokhin",
    author_email='nick@fulminlabs.org',
    packages=['flood_contract_abi'],
    package_data={'flood_contract_abi': ['abis/*.json']},
    url='',
    keywords='',
    install_requires=[],

)