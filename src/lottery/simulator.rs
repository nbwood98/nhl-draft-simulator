use rand::RngExt;

pub struct Simulator;

impl Simulator {
    const PROBABILITIES: &[f32] = &[
        0.185, 0.135, 0.115, 0.095, 0.085, 0.075, 0.065, 0.06,
        0.05, 0.035, 0.03, 0.025, 0.02, 0.015, 0.005, 0.005];
    const BALLS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    const MAX_JUMP: usize = 10;

    fn combination_indexes() -> Vec<usize> {
        (1..=1000).collect()
    }

    fn validate_probabilities() {
        let sum: f32 = Self::PROBABILITIES.iter().sum();
        let epsilon = 1e-6;
        assert!((sum - 1.0).abs() < epsilon, "Probabilities must sum to 1.0, got {}", sum);
    }

    fn build_eligible_teams(initial_order: &[usize]) -> Vec<(usize, f32)> {
        initial_order.iter()
            .enumerate()
            .filter(|(rank, _)| *rank < Self::PROBABILITIES.len())
            .map(|(rank, &team_idx)| (team_idx, Self::PROBABILITIES[rank]))
            .collect()
    }

    fn redistribute_probabilities(
        eligible: &[(usize, f32)],
        locked_teams: &[usize],
    ) -> Vec<(usize, f32)> {
        let remaining: Vec<(usize, f32)> = eligible.iter()
            .filter(|(team_idx, _)| !locked_teams.contains(team_idx))
            .copied()
            .collect();

        let remaining_sum: f32 = remaining.iter().map(|(_, p)| p).sum();

        remaining.iter()
            .map(|&(team_idx, prob)| (team_idx, prob / remaining_sum))
            .collect()
    }

    const DEAD_COMBO: [u8; 4] = [11, 12, 13, 14];

    fn run_single_draw(eligible_teams: &[(usize, f32)]) -> usize {
        let combinations = Self::get_combinations();
        let assigned_combos = Self::assign_combinations_weighted(eligible_teams);

        // Redraw if the dead combination [11,12,13,14] is drawn
        let drawn_balls = loop {
            let mut balls = Self::draw_balls();
            balls.sort_unstable();
            if balls != Self::DEAD_COMBO {
                break balls;
            }
        };

        // Find which combination index the drawn balls correspond to
        let combo_index = combinations.iter().position(|&c| c == drawn_balls)
            .expect("Drawn balls did not match any combination");

        let combo_number = combo_index + 1;

        // Find which team owns that combination number
        assigned_combos.iter()
            .find(|(_, combos)| combos.contains(&combo_number))
            .map(|(team_idx, _)| *team_idx)
            .expect("No team was assigned combination number")
    }

    fn place_winner(
        new_order: &mut Vec<usize>,
        initial_order: &[usize],
        winning_team: usize,
        locked_positions: &[usize],
    ) -> usize {
        let current_pos = initial_order.iter().position(|&t| t == winning_team)
            .expect("Winning team not found in draft order");

        let mut target_pos = current_pos.saturating_sub(Self::MAX_JUMP);
        while locked_positions.contains(&target_pos) {
            target_pos += 1;
        }

        let current_in_new = new_order.iter().position(|&t| t == winning_team)
            .expect("Winning team not found in new order");
        new_order.remove(current_in_new);
        new_order.insert(target_pos, winning_team);
        target_pos
    }

    pub fn quick_simulate(initial_order: Vec<usize>) -> Vec<usize> {
        Self::validate_probabilities();

        let eligible_teams = Self::build_eligible_teams(&initial_order);
        let mut new_order = initial_order.clone();
        let mut locked_positions: Vec<usize> = Vec::new();
        let mut locked_teams: Vec<usize> = Vec::new();

        let round1_winner = Self::run_single_draw(&eligible_teams);
        let round1_winner_original_pos = initial_order.iter()
            .position(|&t| t == round1_winner)
            .expect("Round 1 winner not found in initial order");

        let round1_placed_pos = Self::place_winner(
            &mut new_order, &initial_order, round1_winner, &locked_positions,
        );

        locked_positions.push(round1_placed_pos);
        locked_teams.push(round1_winner);

        if round1_winner_original_pos >= 11 {
            let first_seed = initial_order[0];
            if !locked_teams.contains(&first_seed) {
                locked_positions.push(0);
                locked_teams.push(first_seed);
            }
        }

        let round2_eligible = Self::redistribute_probabilities(&eligible_teams, &locked_teams);
        let round2_winner = Self::run_single_draw(&round2_eligible);

        Self::place_winner(
            &mut new_order, &initial_order, round2_winner, &locked_positions,
        );
        new_order
    }

    fn get_combinations() -> [[u8; 4]; 1000] {
        let mut combinations = [[0u8; 4]; 1000];
        let mut index = 0;

        let balls = Self::BALLS;
        let num_balls = balls.len();

        for (i, &b) in balls.iter().enumerate() {
            if i > num_balls - 4 { break; }

            for (j, &b2) in balls.iter().enumerate().skip(i + 1) {
                if j > num_balls - 3 { break; }

                for (k, &b3) in balls.iter().enumerate().skip(j + 1) {
                    if k > num_balls - 2 { break; }

                    for &b4 in balls.iter().skip(k + 1) {
                        let combo = [b, b2, b3, b4];
                        if combo == [11, 12, 13, 14] { continue; }
                        combinations[index] = combo;
                        index += 1;
                    }
                }
            }
        }
        combinations
    }

    fn assign_combinations_weighted(teams_with_probs: &[(usize, f32)]) -> Vec<(usize, Vec<usize>)> {
        let mut rng = rand::rng();
        let mut available_indexes = Self::combination_indexes();
        let mut assigned: Vec<(usize, Vec<usize>)> = Vec::new();

        for &(team_idx, probability) in teams_with_probs {
            let allotment = (probability * 1000.0).round() as usize;
            let mut team_combos: Vec<usize> = Vec::with_capacity(allotment);

            for _ in 0..allotment {
                if available_indexes.is_empty() { break; }
                let random_pos = rng.random_range(0..available_indexes.len());
                team_combos.push(available_indexes.swap_remove(random_pos));
            }

            assigned.push((team_idx, team_combos));
        }

        // Rounding can leave indexes unassigned — distribute remainders
        while !available_indexes.is_empty() {
            let team = rng.random_range(0..assigned.len());
            let pos = rng.random_range(0..available_indexes.len());
            assigned[team].1.push(available_indexes.swap_remove(pos));
        }

        assigned
    }

    fn draw_balls() -> [u8; 4] {
        let mut rng = rand::rng();
        let ball1 = rng.random_range(1..=14);

        let ball2 = loop {
            let b = rng.random_range(1..=14);
            if b != ball1 { break b; }
        };
        let ball3 = loop {
            let b = rng.random_range(1..=14);
            if b != ball1 && b != ball2 { break b; }
        };
        let ball4 = loop {
            let b = rng.random_range(1..=14);
            if b != ball1 && b != ball2 && b != ball3 { break b; }
        };
        [ball1, ball2, ball3, ball4]
    }
}
