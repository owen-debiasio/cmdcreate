# Packaging cmdcreate

## Files

- **package/create_bin.sh**
  - Builds cmdcreate as a standalone  
- **package/create_deb.sh**
  - Builds cmdcreate as a `.deb` package
- **package/create_rpm.sh**
  - Builds cmdcreate as a `.rpm` package
- **package/create_rpm.sh**
  - Goes through and packages cmdcreate to `.deb`, `.rpm`, and binary files.
- **package/format.sh**
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

- `black` (Python files)
- `shfmt` (Shell files)
- `rustfmt` (Rust files)

---

## Additional notes

> [!NOTE]
> `create_bin.sh`, `create_deb.sh`, and `create_rpm.sh` are all ran in `package.sh`

> [!NOTE]
> Built packages and binaries are built to `~/Downloads/`
