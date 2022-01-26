# ink! Contract Struct Experiment JSON Version

No. Don't do this. It's likely that `serde` or `serde_json` introduces floating point types which makes it not instantiatable.

Run (node version >= 0.5.0 required)
```
substrate-contracts-node --dev --tmp -lerror,runtime::contracts=debug
```

and instantiate the JSON version of the contract and you'll get

```
2022-01-25 17:07:00.005 DEBUG tokio-runtime-worker runtime::contracts: CodeRejected: use of floating point type in locals is forbidden
```

Related issue:  
https://github.com/paritytech/ink/issues/1106