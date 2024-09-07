use std::{cmp::Ordering, hash::{DefaultHasher, Hash, Hasher}};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use project_assignment_lottery_system::{run_lottery, ApplicantId, ApplicantInfo, LotteryConfiguration, ProjectId, ProjectInfo};
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

fn test_ranking(project: ProjectId, a: ApplicantId, b: ApplicantId) -> Ordering {
    let mut a_hash = DefaultHasher::new();
    let mut b_hash = DefaultHasher::new();
    std::format!("{}:{}", project.0, a.0).hash(&mut a_hash);
    std::format!("{}:{}", project.0, b.0).hash(&mut b_hash);
    return a_hash.finish().cmp(&b_hash.finish());
}

fn scale_applicants(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_applicants");
    for num_applicants in [100, 200, 500, 1000, 1500, 2000, 2500].into_iter() {
        let mut rng = StdRng::seed_from_u64(0);

        let num_projects = 150;
        let seats_per_project = num_applicants * 2 / num_projects;
        let preferences_per_applicant = 10;

        let config = LotteryConfiguration {
            ordering_fn: test_ranking,
            projects: Vec::from_iter((0..num_projects).map(|i| ProjectInfo {
                id: ProjectId(i),
                seats: seats_per_project,
            })),
            applicants: Vec::from_iter((0..num_applicants).map(|i| ApplicantInfo {
                id: ApplicantId(i),
                preferences: Vec::from_iter((0..preferences_per_applicant).map(|_| {
                    let val: f64 = rng.sample(Standard);
                    // sqrt distribution to account for the fact that some locations have more applicants than others
                    ProjectId((val.sqrt() * num_projects as f64).floor() as i32)
                }))
            }))
        };

        group.throughput(criterion::Throughput::Elements(num_applicants as u64));
        group.bench_with_input(BenchmarkId::from_parameter(num_applicants),  &config, |b, cfg| {
            b.iter(|| run_lottery(black_box(cfg), &mut rng))
        });
    }
}

fn scale_applicants_ordered_rankings(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_applicants_ordered_rankings");
    for num_applicants in [100, 200, 500, 1000, 1500, 2000, 2500].into_iter() {
        let mut rng = StdRng::seed_from_u64(0);

        let num_projects = 150;
        let seats_per_project = num_applicants * 2 / num_projects;
        let preferences_per_applicant = 10;

        let config = LotteryConfiguration {
            ordering_fn: test_ranking,
            projects: Vec::from_iter((0..num_projects).map(|i| ProjectInfo {
                id: ProjectId(i),
                seats: seats_per_project,
            })),
            applicants: Vec::from_iter((0..num_applicants).map(|i| ApplicantInfo {
                id: ApplicantId(i),
                preferences: Vec::from_iter((0..preferences_per_applicant).map(|_| {
                    let val: f64 = rng.sample(Standard);
                    // sqrt distribution to account for the fact that some locations have more applicants than others
                    ProjectId((val.sqrt() * num_projects as f64).floor() as i32)
                }).sorted_by(|a, b| a.0.cmp(&b.0)))
            }))
        };

        group.throughput(criterion::Throughput::Elements(num_applicants as u64));
        group.bench_with_input(BenchmarkId::from_parameter(num_applicants),  &config, |b, cfg| {
            b.iter(|| run_lottery(black_box(cfg), &mut rng))
        });
    }
}

fn scale_correlation(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_correlation");
    let num_applicants = 2000;
    let num_projects = 150;

    for correlation_amt in [1, 2, 3, 4, 5].into_iter() {
        let mut rng = StdRng::seed_from_u64(0);

        let seats_per_project = num_applicants * 2 / num_projects;
        let preferences_per_applicant = 10;

        let config = LotteryConfiguration {
            ordering_fn: test_ranking,
            projects: Vec::from_iter((0..num_projects).map(|i| ProjectInfo {
                id: ProjectId(i),
                seats: seats_per_project,
            })),
            applicants: Vec::from_iter((0..num_applicants).map(|i| ApplicantInfo {
                id: ApplicantId(i),
                preferences: Vec::from_iter((0..preferences_per_applicant).map(|_| {
                    let val: f64 = rng.sample(Standard);
                    // sqrt distribution to account for the fact that some locations have more applicants than others
                    ProjectId((val.powf(1f64 / correlation_amt as f64) * num_projects as f64).floor() as i32)
                }).sorted_by(|a, b| a.0.cmp(&b.0)))
            }))
        };

        group.throughput(criterion::Throughput::Elements(correlation_amt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(correlation_amt),  &config, |b, cfg| {
            b.iter(|| run_lottery(black_box(cfg), &mut rng))
        });
    }
}

criterion_group!(benches,
    scale_applicants,
    scale_applicants_ordered_rankings,
    scale_correlation,
);
criterion_main!(benches);
