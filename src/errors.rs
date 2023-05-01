use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Point not in curve")]
    PointNotInTheCurve,
}
