mod reports;

pub use reports::write_csv;

pub trait Game: Sized {
    type Params;
    type Results: GameResult;

    fn init(params: &Self::Params) -> Self;
    fn run(self) -> Self::Results;
    fn run_n(params: &Self::Params, n: u64) -> Vec<Self::Results> {
        let mut results = vec![];
        for _ in 0..n {
            let game = Self::init(params);
            results.push(game.run());
        }
        results
    }
}

pub trait GameResult: Sized {
    type Metrics: serde::Serialize + for<'a> serde::Deserialize<'a>;

    fn evaluate(results: Vec<Self>) -> Self::Metrics;
}
