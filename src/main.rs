use {
  anyhow::Error,
  std::{
    io,
    iter::Iterator,
  },
};

fn main() -> Result<(), Error> {
  let max = io::stdin()
    .lines()
    .try_fold(
      Vec::new(),
      |mut l: Vec<u32>, r| -> Result<Vec<u32>, Error> {
        let line = r?;
        if l.is_empty() || line.is_empty() {
          l.push(0)
        } else {
          *l.last_mut().unwrap() += u32::from_str_radix(&line, 10)?;
        }
        Ok(l)
      },
    )?
    .iter()
    .fold([0, 0, 0, 0], |mut topn, x| {
      topn[0] = *x;
      topn.sort();
      topn
    });

  dbg!(max[1..4].iter().sum::<u32>());

  Ok(())
}
