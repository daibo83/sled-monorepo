use std::io;

mod common;

#[test]
#[cfg_attr(miri, ignore)]
fn size_leak() -> io::Result<()> {
    common::setup_logger();

    let tree: sled::Db<1024> =
        sled::Config::tmp()?.flush_every_ms(None).open()?;

    for _ in 0..10_000 {
        tree.insert(b"", b"")?;
    }

    tree.flush()?;

    let sz = tree.size_on_disk()?;
    assert!(
        sz <= 16384,
        "expected system to use less than or equal to \
            16486 bytes, but actually used {}",
        sz
    );

    Ok(())
}
