1. To install xtr, you need to install the GNU Gettext CLI tools into your
   system and launch:
   ```
   cargo install xtr
   ```
2. Either add ```$HOME/.cargo/bin``` to your ```PATH``` or move
   ```$HOME/.cargo/bin/xtr``` into a directory already present in the
   ```PATH```.
3. To get the translation template file, do:
   ```
   xtr src/main.rs -o po/yabinero.pot
   ```
4. Copy that file to ```po/yabinero_<locale>.po```, for example:
   ```
   cp po/yabinero.pot po/yabinero_fr.po
   ```
5. To get the binary catalog, do:
   ```
   mkdir -p locale/<locale>/LC_MESSAGES/
   msgfmt po/yabinero_<locale>.po -o locale/<locale>/LC_MESSAGES/yabinero.mo
   ```
   For instance:
   ```
   mkdir -p locale/fr/LC_MESSAGES/
   msgfmt po/yabinero_fr.po -o locale/fr/LC_MESSAGES/yabinero.mo
   ```
