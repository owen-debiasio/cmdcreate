# Packaging cmdcreate

## Files

- ./create_bin.sh
  - Builds cmdcreate as a standalone  
- ./create_deb.sh
  - Builds cmdcreate as a `.deb` package
- ./create_rpm.sh
  - Builds cmdcreate as a `.rpm` package
- ./format.sh
  - Formats all code
    - Main source code
    - Testing scripts
    - Shell scripts

---

## Dependencies

### Packaging

- `rpm`
- `dpkg`

### Formatting

- `black`
- `shfmt`
- `cargo`

---

## Additional notes

> [!NOTE]
> `create_bin.sh`, `create_deb.sh`, and `create_rpm.sh` are all ran in `package.sh`

> [!NOTE]
> Built packages and binaries are built to `~/Downloads`
