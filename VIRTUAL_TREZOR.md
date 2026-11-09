# Virtual Trezor firmware fork

This fork is based on Trezor Core `2.13.0`, commit
`037e506db454691bda188df2803df2fc26c75f36`, which also contains the legacy
Trezor One `1.14.2` firmware. It supplies the firmware used by
[qpernil/virtual-trezor](https://github.com/qpernil/virtual-trezor).

The integration repository provides the Linux platform adapters, build scripts,
profiles, and tests. Its submodule pins an exact commit of this fork. Builds
consume committed source without applying or reverting patches.

The fork carries these changes as separate commits:

- Linux display, button, supervisor USB, and event-wait hooks, selected by the
  `VIRTUAL_TREZOR_HEADLESS_DISPLAY` and `VIRTUAL_TREZOR_SUPERVISOR_USB` build
  environment flags. The platform source paths and libraries are supplied by
  the integration build.
- OS randomness for the virtual MCU and secure-element paths when
  `VIRTUAL_TREZOR_OS_RNG=1`. These paths read `/dev/urandom`, fail closed on
  entropy errors, and exclude deterministic generators and reseeding APIs.
  Entropy-use flags record completed reads from the same OS entropy source;
  they do not imply independent physical TRNGs or hardware isolation. The
  ordinary upstream emulator retains its deterministic test randomness when
  this opt-in is absent.
- Const-trait syntax compatible with the tested stable Rust `1.99.1` compiler.
  The integration build uses `RUSTC_BOOTSTRAP=1` for upstream experimental
  language features. Compiler upgrades require revalidation.

The legacy integration replaces platform objects at link time. Safe 3 uses
model `T3B1`; the Linux worker is not production-authenticated Trezor hardware.

To update the base, import a selected official release from the read-only
`upstream` remote, resolve and review the fork's changes, and validate them
through the integration repository before advancing its submodule pin. Keep
upstream release tags unchanged. The local source of record is the standalone
Mac checkout; lab checkouts consume published commits through Git.
