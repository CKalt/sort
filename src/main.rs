#[derive(Debug)]
#[allow(dead_code)]
struct Chromosome {
    pub actions: String,
    pub fitness: f64,
}
impl Chromosome {
    fn new(actions: &str, fitness: f64) -> Self {
        Self {
            actions: actions.into(),
            fitness,
        }
    }
}

type Population = Vec<Chromosome>;

#[derive(Debug)]
struct Ga {
    population: Option<Population>,
}
impl Ga {
    fn display(&self, msg: &str) {
        println!("{}\n", msg);
        for (row, chr) in self.population
                    .as_ref().unwrap().iter().enumerate() {
            println!("row={}, actions={}, fitness={}",  row, 
                     chr.actions,
                     chr.fitness);
        }
        println!("\n");
    }
    fn new() -> Self {
        Self {
            population: None,
        }
    }
    fn generate_population(&mut self) {
        let mut pop: Population = Vec::new();
        pop.push(Chromosome::new("abc", 70.3));
        pop.push(Chromosome::new("def", 90.2));
        pop.push(Chromosome::new("xyz", 23.5));
        pop.push(Chromosome::new("ijk", 70.32));
        self.population = Some(pop);
    }
    fn sort_population(&mut self) {
        let pop = self.population.as_mut().unwrap();
        pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }
}

fn main() {
    let mut ga = Ga::new();
    ga.generate_population();
    
//    println!("ga = {:?}", ga);
    
    ga.display("before sort");

    ga.sort_population();
    
//    println!("ga = {:?}", ga);

    ga.display("after sort");
}
