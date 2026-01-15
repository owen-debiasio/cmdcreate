# Features, fixes, and changes to be applied to cmdcreate

- **testing/install.sh**
  - Add more flexibility with removing current cmdcreate version
  - Works with `pacman`, `apt`, and `dnf`

- **package/create_deb.sh**
  - Fix error by adding dependency:
     - `cmdcreate: error while loading shared libraries: libssl.so.3: cannot open shared object file: No such file or directory`
