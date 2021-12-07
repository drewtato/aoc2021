#[derive(Debug, Clone)]
pub struct CellularAutomaton<G, R> {
	pub grid: G,
	rule: R,
}

impl<G, R> CellularAutomaton<G, R> {
	pub fn new(grid: G, rule: R) -> Self {
		Self { grid, rule }
	}
}

impl<G, R, P, N> CellularAutomaton<G, R>
where
	G: Grid<Pos = P>,
	R: Fn(&P, &G) -> N,
{
}

trait Grid {
	type Pos;
}
