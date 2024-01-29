#[cfg(test)]
mod tests {
    use trdelnik_client::Config;
    use trdelnik_client::Fuzz;
    use trdelnik_client::Test;

    //     use fuzz_example3::entry;
    // use fuzz_instructions::fuzz_example3_fuzz_instructions::{FuzzInstruction, InitVesting};
    // use program_client::fuzz_example3_instruction::*;
    // use trdelnik_client::{fuzz_trd, fuzzing::*};
    // mod accounts_snapshots;
    // mod fuzz_instructions;

    #[test]
    fn test_merge_and_precedence1() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_fuzz_args(String::default());
        assert_eq!(env_var_string, "-t 10 -N 0 ");
    }
    #[test]
    fn test_merge_and_precedence2() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_fuzz_args("-t 0 -N10 --exit_upon_crash".to_string());

        assert_eq!(env_var_string, "-t 10 -N 0 -t 0 -N10 --exit_upon_crash");
    }
    #[test]
    fn test_merge_and_precedence3() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };
        let env_var_string =
            config.get_fuzz_args("-t 100 -N 5000 -Q -v --exit_upon_crash".to_string());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -t 100 -N 5000 -Q -v --exit_upon_crash"
        );
    }
    #[test]
    fn test_merge_and_precedence4() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_fuzz_args("-t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666".to_string());
        assert_eq!(
            env_var_string,
            "-t 10 -N 0 -t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
}
