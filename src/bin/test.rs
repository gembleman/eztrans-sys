use eztrans_sys::{EzTransError, EzTransLib};

fn main() -> Result<(), EzTransError> {
    let ez_trans = EzTransLib::new(None)?;
    ez_trans.initialize(None, None)?;

    const TEXT: &str = "가나다라おはようございます。";

    for i in 0..50 {
        let transleted = ez_trans.translate(TEXT)?;
        // println!("{}: {}", i, transleted);
    }

    ez_trans.terminate()?;

    Ok(())
}
