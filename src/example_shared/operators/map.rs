use communication::*;
use communication::pact::Pipeline;

use example_shared::*;
use example_shared::operators::unary::UnaryStreamExt;

pub trait MapExt<G: GraphBuilder, D1: Data> {
    fn map<D2: Data, L: Fn(D1)->D2+'static>(&self, logic: L) -> Stream<G, D2>;
}

impl<G: GraphBuilder, D1: Data> MapExt<G, D1> for Stream<G, D1> {
    fn map<D2: Data, L: Fn(D1)->D2+'static>(&self, logic: L) -> Stream<G, D2> {
        self.unary_stream(Pipeline, format!("Map"), move |input, output| {
            while let Some((time, data)) = input.pull() {
                output.give_at(&time, data.drain(..).map(|x| logic(x)));
            }
        })
    }
}
