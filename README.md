# csv_to_json_rs
Arch Linux Repository Building CLI
----

# ⚙ Build
```shell
cargo build --release
cp -rf ./target/release/arb /usr/bin
```

# Usage
```shell
arb --help
arb [-c|--config-file [config_file_path]] [-s | --show-all] <aur|official|custom> <package_name> [package_path] [-a|--add] [-r|--remove]
```

----
## 💳 License

MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)
