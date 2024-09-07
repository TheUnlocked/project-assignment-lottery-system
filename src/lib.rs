use std::{cmp::Ordering, collections::HashMap};

use binary_heap_plus::BinaryHeap;
use compare::Compare;
use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicantId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(pub i32);

#[derive(Debug, Clone)]
pub struct ApplicantInfo {
    pub id: ApplicantId,
    pub preferences: Vec<ProjectId>,
}

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub id: ProjectId,
    pub seats: i32,
}

#[derive(Debug, Clone)]
pub struct LotteryConfiguration {
    pub applicants: Vec<ApplicantInfo>,
    pub projects: Vec<ProjectInfo>,
    pub ordering_fn: fn (project: ProjectId, a: ApplicantId, b: ApplicantId) -> Ordering,
}

#[derive(Debug)]
pub struct LotteryResult {
    pub assignments: HashMap<ProjectId, Vec<ApplicantId>>,
    pub unassigned: Vec<ApplicantId>,
}

pub fn run_lottery<R : Rng>(config: &LotteryConfiguration, rng: &mut R) -> LotteryResult {
    let mut applicants = config.applicants.clone();
    applicants.shuffle(rng);
    return run_assignments(&LotteryConfiguration { applicants, ..config.clone() });
}

struct ScoringComparator(ProjectId, fn (project: ProjectId, a: ApplicantId, b: ApplicantId) -> Ordering);

impl Compare<ApplicantInfo> for ScoringComparator {
    fn compare(&self, l: &ApplicantInfo, r: &ApplicantInfo) -> Ordering {
        self.1(self.0, r.id, l.id)
    }
}

// Assignments algorithm pseudo-code description:
//      (implementation varies slightly)
//
// while applicants remain in the queue:
//     app <- get the next applicant
//     proj <- pop the highest-ranked project from app's preferences
//
//     if proj does not exist:
//         mark app as unassigned and pop them from the queue
//     else if proj has slots left:
//         put app into proj and pop them from the queue
//     else:
//         weak <- get the lowest-ranked applicant in proj
//         if rank(proj, app, weak) finds app is a stronger applicant:
//             swap app and weak (app goes into proj and weak goes to the front of the queue)
//         else:
//             (just continue, app stays at the front of the queue)

pub fn run_assignments(config: &LotteryConfiguration) -> LotteryResult {
    let mut to_assign: Vec<ApplicantInfo> = config.applicants.iter().map(|x| {
        ApplicantInfo { id: x.id, preferences: x.preferences.iter().rev().cloned().collect::<Vec<ProjectId>>() }
    }).collect();

    let mut unassigned = Vec::<ApplicantId>::new();
    let mut project_assignments = HashMap::<ProjectId, (i32, BinaryHeap<ApplicantInfo, ScoringComparator>)>::new();

    for project in config.projects.iter() {
        project_assignments.insert(project.id, (project.seats, BinaryHeap::from_vec_cmp(vec![], ScoringComparator(project.id, config.ordering_fn))));
    }

    'assignments: loop {
        match to_assign.last_mut() {
            None => break,
            Some(assign_next) => {
                loop {
                    match assign_next.preferences.pop() {
                        Some(project) => {
                            let (seats, heap) = project_assignments.get_mut(&project).unwrap();
                            if heap.len() < *seats as usize {
                                heap.push(assign_next.clone());
                                break;
                            }
                            match heap.peek() {
                                Some(weakest) => 
                                    match (config.ordering_fn)(project, assign_next.id, weakest.id) {
                                        Ordering::Greater => {
                                            // After replacing the weaker candidate we want to give the person
                                            // we just removed from the project the first spot in the queue,
                                            // so we swap them into the last position in the to_assign vec and
                                            // restart the loop without popping them.
                                            let weakest_value = heap.pop().unwrap().clone();
                                            heap.push(assign_next.clone());
                                            *assign_next = weakest_value;
                                            continue 'assignments;
                                        }
                                        _ => {}
                                    }
                                _ => {}
                            }
                        }
                        None => {
                            unassigned.push(assign_next.id);
                            break;
                        }
                    }
                }
            }
        }

        // Once we finish assigning a user, pop them off the queue.
        to_assign.pop();
    }

    return LotteryResult {
        unassigned,
        assignments: project_assignments.iter_mut()
            .map(|(id, (_, heap))| (
                *id,
                heap.iter().map(|app| app.id).collect(),
            ))
            .collect(),
    };
}
