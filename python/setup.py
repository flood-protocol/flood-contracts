from setuptools import setup, find_packages
from os import environ

setup(
    name='flood-abi',
    version=environ['GIT_TAG'],
    license='MIT',
    author="Nick Erokhin",
    author_email='nick@fulminlabs.org',
    packages=find_packages('src'),
    package_dir={'': 'src'},
    url='',
    keywords='',
    install_requires=[],

)