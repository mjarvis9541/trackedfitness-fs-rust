impl ExerciseQueryWithPrevious {
pub fn get_detail_href(&self) -> String {
format!(
"/users/{}/workouts/{}/{}/{}",
self.username, self.date, self.workout_id, self.exercise_id,
)
}
pub fn get_next_set_weight(&self) -> String {
let last = self
.sets
.last()
.map(|last_set| format!("{:.2}", last_set.weight))
.unwrap_or_default();

        todo!()
    }

}
