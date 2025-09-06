```bash
# deploy
mergerfs -o defaults,use_ino,category.create=ff,category.action=epff \
  ./overwrite:./mod2:./mod1:./game \
  ./view
# disable deployed
fusermount -zu ./view
```
