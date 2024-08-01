from setuptools import setup, find_packages

setup(
    name="opendrift_simulation",
    version="0.1.0",
    packages=find_packages(),
    include_package_data=True,
    install_requires=[
        "aiohttp",
        "grpcio",
        "grpcio-tools",
        "opendrift",
        "protobuf",
        "gdal",  # Ensure GDAL is listed here
    ],
    entry_points={
        'console_scripts': [
            'generate_protobuf=scripts.generate_protobuf:main',
        ],
    },
)
