pub trait Game: Sized {
    type Params;
    type Results;

    fn init(params: &Self::Params) -> Self;
    fn run(self, debug: bool) -> Self::Results;
    fn run_n(params: &Self::Params, n: u64, debug: bool) -> Vec<Self::Results> {
        let mut results = vec![];
        for _ in 0..n {
            let game = Self::init(params);
            results.push(game.run(debug));
        }
        results
    }
}
