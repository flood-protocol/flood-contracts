from setuptools import setup, find_packages
from os import environ

setup(
    name='flood_contract_abi',
    version=environ['GIT_TAG'],
    license='MIT',
    author="Nick Erokhin",
    author_email='nick@fulminlabs.org',
    packages=['flood_contract_abi'],
    package_data={'flood_contract_abi': ['abis/*.json']},
    url='',
    keywords='',
    install_requires=[],

)