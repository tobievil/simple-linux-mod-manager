```bash
# deploy
mergerfs -o defaults,use_ino,category.create=ff,category.action=epff \
  ./overwrite:./mod2:./mod1:./game \
  ./view
# disable deployed
fusermount -zu ./view

cargo run -- create -c ./tmp/config.json -g ./tmp/game -m ./tmp/mods
cargo run -- add-mod -c ./tmp/config.json --name mod1 --version 1.0 --mod-path ./tmp/raw_mod/mod1
cargo run -- view -c ./tmp/config.json
```
