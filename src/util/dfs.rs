use std::fmt::Debug;

pub trait Dfs {
    type Score: Ord;
    type Action: Clone + Debug;
    type Actions: Iterator<Item = Self::Action>;
    fn score(&self) -> Self::Score;
    fn actions(&self) -> Self::Actions;
    fn perform(&mut self, history: &Vec<Self::Action>, action: &Self::Action) -> bool;
    fn backtrack(&mut self, history: &Vec<Self::Action>, action: &Self::Action);

    fn dfs(&mut self) -> Self::Score {
        let mut history: Vec<Self::Action> = Vec::new();
        let mut stack: Vec<Self::Actions> = vec![self.actions()];
        let mut best_score = self.score();
        while let Some(actions) = stack.last_mut() {
            if let Some(action) = actions.next() {
                if self.perform(&history, &action) {
                    history.push(action);
                    stack.push(self.actions());
                }
            } else {
                best_score = best_score.max(self.score());
                if let Some(prev_action) = history.pop() {
                    self.backtrack(&history, &prev_action);
                }
                stack.pop();
            }
        }
        best_score
    }
}
