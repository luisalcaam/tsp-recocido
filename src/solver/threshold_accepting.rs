use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub trait ThresholdAcceptingProblem{
    type Solution: Clone;

    fn objective_function(&self, solution: &Self::Solution) -> f64;

    fn neighbor<R: Rng + ?Sized>(&self, solution: &Self::Solution, rng: &mut R) -> Self::Solution;
}

pub struct ThresholdAccepting {
    pub l: usize,
    pub max_attempts: usize,
    pub epsilon: f64,
    pub phi: f64,
    pub p_target: f64,
    pub n_samples: usize,
    pub epsilon_p: f64,
}

impl ThresholdAccepting {
    pub fn new(
        l: usize,
        max_attempts: usize,
        epsilon: f64,
        phi: f64,
        p_target: f64,
        n_samples: usize,
        epsilon_p: f64,
    ) -> Self {
        Self {
            l,
            max_attempts,
            epsilon,
            phi,
            p_target,
            n_samples,
            epsilon_p,
        }
    }

    pub fn calcula_lote<P: ThresholdAcceptingProblem, R: Rng + ?Sized>(&self,
                                                                       problem: &P,
                                                                       mut s: P::Solution,
                                                                       t: f64,
                                                                       rng: &mut R,
    ) -> (f64, P::Solution) {
        let mut c = 0;
        let mut r = 0.0;
        let mut attempts = 0;
        while c < self.l && attempts < self.max_attempts {
            attempts += 1;
            let s_prime = problem.neighbor(&s, rng);
            let f_s = problem.objective_function(&s);
            let f_s_prime = problem.objective_function(&s_prime);

            if f_s_prime <= f_s + t {
                s = s_prime;
                c += 1;
                r += f_s_prime;
            }
        }
        let avg = if c > 0 { r / (self.l as f64) } else { 0.0 };
        (avg, s)
    }


    pub fn per_accepted<P: ThresholdAcceptingProblem, R: Rng + ?Sized>(&self,
                                                                       problem: &P,
                                                                       s: &P::Solution,
                                                                       t: f64,
                                                                       rng: &mut R,
    ) -> f64 {
        let mut cnt = 0;
        let f_s = problem.objective_function(s);
        
        for _ in 0..self.n_samples {
            let s_prime = problem.neighbor(s, rng);
            if problem.objective_function(&s_prime) <= f_s + t {
                cnt += 1;
        
            }
        }
        (cnt as f64) / (self.n_samples as f64)
    }

    

    pub fn binary_search<P: ThresholdAcceptingProblem, R: Rng + ?Sized>(&self,
                                                                        problem: &P,
                                                                        s: &P::Solution,
                                                                        t1: f64,
                                                                        t2: f64,
                                                                        rng: &mut R,
    ) -> f64 {
        let t_m = (t1 + t2) / 2.0;
        if (t2 - t1).abs() < self.epsilon_p {
            return t_m;
        }
        let p_m = self.per_accepted(problem, s, t_m, rng);
        if (self.p_target - p_m).abs() < self.epsilon_p {
            return t_m;
        }
        if p_m > self.p_target {
            self.binary_search(problem, s, t1, t_m, rng)
        } else {
            self.binary_search(problem, s, t_m, t2, rng)
        }
    }



    pub fn temp_init<P: ThresholdAcceptingProblem, R: Rng + ?Sized>(&self,
                                                                    problem: &P,
                                                                    s: &P::Solution,
                                                                    mut t_init: f64,
                                                                    rng: &mut R,
    ) -> f64 {
        let mut p = self.per_accepted(problem, s, t_init, rng);
        if (self.p_target - p).abs() <= self.epsilon_p {
            return t_init;
        }

        let (t1,t2);
        if p < self.p_target {
            while p < self.p_target {
                t_init *= 2.0;
                p = self.per_accepted(problem, s, t_init, rng);
            }
            t1 = t_init / 2.0;
            t2 = t_init;
        } else {
            while p > self.p_target {
                t_init /= 2.0;
                p = self.per_accepted(problem, s, t_init, rng);
            }
            t1 = t_init;
            t2 = t_init * 2.0;
        }
        
        self.binary_search(problem, s, t1, t2,rng)
        
    }

    pub fn solve<P: ThresholdAcceptingProblem>(&self,
                                               problem: &P,
                                               initial_solution: P::Solution,
                                               initial_t_guess: f64,
                                               seed: u64,
    ) -> P::Solution {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let mut t = self.temp_init(problem, &initial_solution, initial_t_guess, &mut rng);
        let mut s = initial_solution;

        let mut best_solution = s.clone();
        let mut best_score = problem.objective_function(&best_solution);

        let mut p = 0.0;

        let mut iter_lote = 0;
        
        while t > self.epsilon { 
            let mut q = f64::INFINITY;
            while p < q {
                q = p;
                let (avg_cost,new_s) = self.calcula_lote(problem, s.clone(), t, &mut rng);
                p = avg_cost;
                s = new_s;

                let cur_score = problem.objective_function(&s);

                if cur_score < best_score {
                    best_score = cur_score;
                    best_solution = s.clone();
                }
            }

            iter_lote += 1;


            println!("[Lote {:4}] T: {:10.10} | Prom. Aceptados: {:10.2} | Mejor Costo: {:10.2}",
                iter_lote, t, p, best_score );
            
            t *= self.phi;
        }
        best_solution
    }
    
}


