use clap::{Parser, Subcommand, ValueEnum};
use estate_opt_core::{HardConstraints, Property, load_properties_csv};
use estate_opt_scoring::ScoreWeights;
use estate_opt_solvers::{
    RankedProperty, annealing_rank, generate_synthetic_properties, greedy_rank,
};

#[derive(Parser)]
#[command(name = "estate-opt")]
#[command(about = "Real estate optimization CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rank {
        file: String,
        #[arg(long)]
        city: Option<String>,
        #[arg(long)]
        budget: Option<f64>,
        #[arg(long, default_value_t = 5)]
        top: usize,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long, value_enum, default_value_t = SolverKind::Greedy)]
        solver: SolverKind,
    },
    DemoRank {
        #[arg(long)]
        city: Option<String>,
        #[arg(long)]
        budget: Option<f64>,
        #[arg(long, default_value_t = 100)]
        count: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long, value_enum, default_value_t = SolverKind::Greedy)]
        solver: SolverKind,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum SolverKind {
    Greedy,
    Annealing,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rank {
            file,
            city,
            budget,
            top,
            format,
            solver,
        } => {
            let properties = load_properties_csv(file).expect("load property csv");
            run_rank(properties, city, budget, top, format, solver);
        }
        Commands::DemoRank {
            city,
            budget,
            count,
            seed,
            format,
            solver,
        } => {
            let properties = generate_synthetic_properties(count, seed);
            run_rank(properties, city, budget, 5, format, solver);
        }
    }
}

fn run_rank(
    properties: Vec<Property>,
    city: Option<String>,
    budget: Option<f64>,
    top: usize,
    format: OutputFormat,
    solver: SolverKind,
) {
    let constraints = HardConstraints {
        max_budget: budget,
        city,
        min_liquidity_score: None,
        max_vacancy_risk: None,
    };

    let ranked = match solver {
        SolverKind::Greedy => greedy_rank(&properties, &constraints, &ScoreWeights::default()),
        SolverKind::Annealing => {
            annealing_rank(&properties, &constraints, &ScoreWeights::default(), 42, 250)
        }
    };

    render_output(&ranked.into_iter().take(top).collect::<Vec<_>>(), format);
}

fn render_output(items: &[RankedProperty], format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            for item in items {
                println!(
                    "{} | {} | score {:.3} | {}",
                    item.property.id,
                    item.property.locality,
                    item.breakdown.total,
                    item.explanation.summary
                );
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(items).expect("json output")
            );
        }
        OutputFormat::Markdown => {
            println!("| id | locality | score | summary |");
            println!("| --- | --- | ---: | --- |");
            for item in items {
                println!(
                    "| {} | {} | {:.3} | {} |",
                    item.property.id,
                    item.property.locality,
                    item.breakdown.total,
                    item.explanation.summary.replace('|', "/")
                );
            }
        }
    }
}
