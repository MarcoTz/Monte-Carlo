pub trait Game: Sized {
    type Params;
    type Results;

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
