## SSH Remote

Simple tool to retrieve your ssh hosts using cli.

### Usage

1. Touch file `hostnames.yaml` with simillar contents:
```yaml
some_service:
  test:
    - some_test_host.site
  prod:
    - some_prod_host.site

...
```
2. Build the executable using `cargo`: 
```bash
cargo build --release
```
3. Execute command to get the appropriate host
```bash 
ssh_remote
```

Such output will be provided:
```
Pick option from: 
1. some_service
1
Pick option from: 
1. test
2. prod
2

----------
ssh some_prod_host.site
```

Additional arguments:
* `--verbose` - will print config structure before interaction
* `--dry` - won't copy resulted command into your clipboard