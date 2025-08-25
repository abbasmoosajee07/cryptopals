from setuptools import setup, find_packages

setup(
    name='challenge_utils',
    version='0.1',
    packages=find_packages(),
    install_requires=[
        'psutil',         # For memory and process monitoring
        'numpy',          # For numerical operations
        'pandas',         # For data manipulation and analysis
        'matplotlib',     # For plotting
    ],
)
