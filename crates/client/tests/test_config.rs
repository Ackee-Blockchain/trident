#[cfg(test)]
mod tests {
    use trdelnik_client::__private::Config;
    use trdelnik_client::__private::Fuzz;
    use trdelnik_client::__private::Test;
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
