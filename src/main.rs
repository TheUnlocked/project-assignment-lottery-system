use std::cmp::Ordering;

use project_assignment_lottery_system::*;
use rand::thread_rng;

fn main() {
    let config = LotteryConfiguration {
        ordering_fn: |_, _, _| Ordering::Equal,
        projects: vec![
            ProjectInfo { id: ProjectId(0), seats: 2 },
            ProjectInfo { id: ProjectId(1), seats: 3 },
        ],
        applicants: vec![
            ApplicantInfo { id: ApplicantId(0), preferences: vec![ProjectId(0)] },
            ApplicantInfo { id: ApplicantId(1), preferences: vec![ProjectId(1), ProjectId(0)] },
            ApplicantInfo { id: ApplicantId(2), preferences: vec![ProjectId(0), ProjectId(1)] },
            ApplicantInfo { id: ApplicantId(3), preferences: vec![ProjectId(0)] },
            ApplicantInfo { id: ApplicantId(4), preferences: vec![ProjectId(1)] },
            ApplicantInfo { id: ApplicantId(5), preferences: vec![ProjectId(1), ProjectId(0)] },
            ApplicantInfo { id: ApplicantId(6), preferences: vec![ProjectId(0)] },
        ].into_iter().rev().collect()
    };
    let result = run_assignments(&config);
    println!("No randomization: {:?}", result);
    let result = run_lottery(&config, &mut thread_rng());
    println!("Lottery: {:?}", result);
}
