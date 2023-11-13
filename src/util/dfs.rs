use std::fmt::Debug;

pub trait Dfs {
    type Score: Ord + Debug;
    type Action: Clone + Debug;
    type Actions: Iterator<Item = Self::Action>;
    type Trace: Debug;
    fn score(&self) -> Self::Score;
    fn actions(&self) -> Self::Actions;
    fn perform(&mut self, history: &[Self::Trace], action: &Self::Action)
        -> Option<Self::Trace>;
    fn backtrack(&mut self, history: &[Self::Trace], trace: Self::Trace);

    fn dfs(&mut self) -> Self::Score {
        let mut history: Vec<Self::Trace> = Vec::new();
        let mut stack: Vec<Self::Actions> = vec![self.actions()];
        let mut best_score = self.score();
        while let Some(actions) = stack.last_mut() {
            if let Some(action) = actions.next() {
                if let Some(trace) = self.perform(&history, &action) {
                    history.push(trace);
                    stack.push(self.actions());
                }
            } else {
                best_score = best_score.max(self.score());
                if let Some(trace) = history.pop() {
                    self.backtrack(&history, trace);
                }
                stack.pop();
            }
        }
        best_score
    }
}
