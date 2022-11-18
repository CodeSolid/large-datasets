# Python Large Datasets

This is the companion source for the article [Large Data Sets in Python: Pandas and the Alternatives](https://codesolid.com/large-data-sets-in-python-pandas-and-the-alternatives/).


## Installing the tools:

This project was originally created as follows, but see also the requirements.txt and environment.yml for more precise versions:

```
conda create -n python3.11 python=3.11
conda activate python3.11
python -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install jupyterlab ipython pandas dask polars fastparquet
```
