use std::{cmp::Ordering, collections::{HashMap, HashSet}, hash::{DefaultHasher, Hash, Hasher}};

use project_assignment_lottery_system::{run_assignments, run_lottery, ApplicantId, ApplicantInfo, LotteryConfiguration, ProjectId, ProjectInfo};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn test_ranking(project: ProjectId, a: ApplicantId, b: ApplicantId) -> Ordering {
    let mut a_hash = DefaultHasher::new();
    let mut b_hash = DefaultHasher::new();
    std::format!("{}:{}", project.0, a.0).hash(&mut a_hash);
    std::format!("{}:{}", project.0, b.0).hash(&mut b_hash);
    return a_hash.finish().cmp(&b_hash.finish());
}

#[test]
fn shuffling_has_no_impact_with_non_equal_ranking() {
    let mut rng = StdRng::seed_from_u64(0);

    let num_applicants = 1501;
    let num_projects = 100;
    let seats_per_project = 15;
    let preferences_per_applicant = 5;

    let config = LotteryConfiguration {
        ordering_fn: test_ranking,
        projects: Vec::from_iter((0..num_projects).map(|i| ProjectInfo {
            id: ProjectId(i),
            seats: seats_per_project,
        })),
        applicants: Vec::from_iter((0..num_applicants).map(|i| ApplicantInfo {
            id: ApplicantId(i),
            preferences: Vec::from_iter((0..preferences_per_applicant).map(|_| {
                ProjectId(rng.sample(Uniform::new(0, num_projects)))
            }))
        }))
    };

    let no_shuffle_result = run_assignments(&config);
    let shuffle_result = run_lottery(&config, &mut rng);

    assert_eq!(
        no_shuffle_result.unassigned.into_iter().collect::<HashSet<ApplicantId>>(),
        shuffle_result.unassigned.into_iter().collect()
    );
    assert_eq!(
        no_shuffle_result.assignments.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect::<HashMap<ProjectId, HashSet<ApplicantId>>>(),
        shuffle_result.assignments.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()
    );
}

#[test]
fn shuffling_has_impact_with_equal_ranking() {
    let mut rng = StdRng::seed_from_u64(0);

    let num_applicants = 1501;
    let num_projects = 100;
    let seats_per_project = 15;
    let preferences_per_applicant = 5;

    let config = LotteryConfiguration {
        ordering_fn: |_, _, _| Ordering::Equal,
        projects: Vec::from_iter((0..num_projects).map(|i| ProjectInfo {
            id: ProjectId(i),
            seats: seats_per_project,
        })),
        applicants: Vec::from_iter((0..num_applicants).map(|i| ApplicantInfo {
            id: ApplicantId(i),
            preferences: Vec::from_iter((0..preferences_per_applicant).map(|_| {
                ProjectId(rng.sample(Uniform::new(0, num_projects)))
            }))
        }))
    };

    let no_shuffle_result = run_assignments(&config);
    let shuffle_result = run_lottery(&config, &mut rng);

    assert_ne!(
        no_shuffle_result.unassigned.into_iter().collect::<HashSet<ApplicantId>>(),
        shuffle_result.unassigned.into_iter().collect()
    );
    assert_ne!(
        no_shuffle_result.assignments.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect::<HashMap<ProjectId, HashSet<ApplicantId>>>(),
        shuffle_result.assignments.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()
    );
}
