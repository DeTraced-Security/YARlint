from setuptools import setup, find_packages

setup(
    name='yarlinter',
    version='1.0',
    packages=find_packages(),
    install_requires=[
        'yarlinter.config',
        'yarlinter.rules.base',
        'yarlinter.utils'
    ]
)