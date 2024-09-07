This is a prototype algorithm for assigning applicants to projects based on the applicants' ranked preferences.
Unlike [other solutions](https://en.m.wikipedia.org/wiki/Rank-maximal_allocation) for similar problems, this one
foregoes the goal of maximizing the number of people who get high-ranked placements, instead focusing on making the
algorithm impossible to game.

The particular scenario this algorithm is trying to solve is one where applicants are not required to rank every project
and they cannot be assigned to projects they do not rank. A problem with other solutions is that because their goal is
typically to get everyone a spot, including a low-demand project in one's rankings will almost guarantee the applicant
to get that low-demand location, since other more strateigic applicants would have only ranked high-demand projects.
This results in the dilemma of people ranking projects to game the system rather than actually ranking projects in order
of preference.

This new algorithm (called PALS for Project Assignment Lottery System) has the beneficial property that the most optimal
strategy for an applicant is to rank projects in the order that they actually want the projects. In other words,
attempting to game the system can only lead to an equal or worse outcome for the applicant. This comes at the cost of
potentially having more applicants completely unassigned at the end, with all of the spots in the projects they ranked
being full. However, because there is no longer any advantage to gaming the system, I believe that it would incentivize
applicants to actually rank every single project they might be interested in, rather than leaving off low-demand
projects as they might do when faced with alternative assignment algorithms.

The algorithm is described in pseudo-code in [`src/lib.rs`](src/lib.rs). I do not have any formal proofs for it, but it
does seem to pass the "vibe" check.

The tests and benchmarks are not intended to be even remotely comprehensive, and were mostly written to discover and
sanity-check certain properties. Lots of code in there is hacked together and copy-pasted.
