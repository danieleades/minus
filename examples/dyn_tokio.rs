use futures::join;
use tokio::time::sleep;

use std::fmt::Write;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = minus::Pager::default_dynamic();

    let increment = async {
        for i in 0..=30_u32 {
            let mut output = output.lock().unwrap();
            writeln!(output.lines, "{}", i)?;
            drop(output);
            sleep(Duration::from_millis(100)).await;
        }
        Result::<_, std::fmt::Error>::Ok(())
    };

    let (res1, res2) = join!(minus::tokio_updating(output.clone()), increment);
    res1?;
    res2?;
    Ok(())
}
