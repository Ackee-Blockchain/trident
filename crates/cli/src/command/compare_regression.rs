use anyhow::Error;
use fehler::throws;
use trident_fuzz_metrics::compare_regression_files;
use trident_fuzz_metrics::ComparisonResult;

#[throws]
pub fn compare_regression(file1: String, file2: String) {
    println!("🔍 Comparing regression files:");
    println!("  File 1: {}", file1);
    println!("  File 2: {}", file2);
    println!();

    let result = compare_regression_files(&file1, &file2)?;

    if result.identical {
        println!("✅ \x1b[32mRegression files are identical!\x1b[0m");
        println!("   All iteration seeds have matching account states.");
    } else {
        println!("❌ \x1b[31mRegression files differ!\x1b[0m");
        println!();

        if !result.differing_seeds.is_empty() {
            println!("🔄 \x1b[33mIteration seeds with different states:\x1b[0m");
            for seed in &result.differing_seeds {
                println!("  • {}", seed);
            }
            println!(
                "   {} differing seed(s) found",
                result.differing_seeds.len()
            );
            println!();
        }

        if !result.only_in_first.is_empty() {
            println!("📄 \x1b[34mSeeds only in first file:\x1b[0m");
            for seed in &result.only_in_first {
                println!("  • {}", seed);
            }
            println!(
                "   {} unique seed(s) in first file",
                result.only_in_first.len()
            );
            println!();
        }

        if !result.only_in_second.is_empty() {
            println!("📄 \x1b[34mSeeds only in second file:\x1b[0m");
            for seed in &result.only_in_second {
                println!("  • {}", seed);
            }
            println!(
                "   {} unique seed(s) in second file",
                result.only_in_second.len()
            );
            println!();
        }

        print_detailed_comparison(&result);

        // Exit with non-zero code if differences found
        std::process::exit(1);
    }
}

fn print_detailed_comparison(result: &ComparisonResult) {
    println!("📊 \x1b[36mDetailed Summary:\x1b[0m");
    println!(
        "  ✓ Total differing seeds: {}",
        result.differing_seeds.len()
    );
    println!("  ✓ Seeds only in file 1: {}", result.only_in_first.len());
    println!("  ✓ Seeds only in file 2: {}", result.only_in_second.len());

    let total_differences =
        result.differing_seeds.len() + result.only_in_first.len() + result.only_in_second.len();
    println!("  ✓ Total differences: {}", total_differences);

    if total_differences > 0 {
        println!();
        println!("🎯 \x1b[33mRecommendations:\x1b[0m");
        if !result.differing_seeds.is_empty() {
            println!("  • Investigate state changes for differing seeds");
            println!("  • Check if account mutations differ between test runs");
        }
        if !result.only_in_first.is_empty() || !result.only_in_second.is_empty() {
            println!("  • Verify that both test runs covered the same scenarios");
            println!("  • Check if different master seeds were used");
        }
    }
}
