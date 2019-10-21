import pandas as pd

df = pd.read_csv("~/Desktop/preq-ngin/all_reqs.csv", header=None)

df.fillna("").to_csv(
    "~/Desktop/preq-ngin/src/reqs2.csv", 
    sep='|',
    index=False
)