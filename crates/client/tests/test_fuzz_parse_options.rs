#[cfg(test)]
mod tests {
    #[test]
    fn test_cmd_options_parsing() {
        let mut command = String::from("-Q -v --extension fuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -e fuzz -v");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v --extension ");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q --extension fuzz -v -e ");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        let mut command = String::from("--extension buzz -e fuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -e fuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -efuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --extfuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --workspace");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v -e");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = trdelnik_client::get_cmd_option_value(args, "-e", "");
        assert_eq!(extension, None);
    }

    #[test]
    fn test_get_crash_dir_and_ext() {
        pub const TARGET: &str = "fuzz_0";
        pub const TEST_CRASH_PATH: &str = "/home/fuzz/test-crash-path";

        const ROOT: &str = "/home/fuzz/";

        let default_crash_path =
            std::path::Path::new(trdelnik_client::HFUZZ_WORKSPACE_DEFAULT).join(TARGET);
        let env_specified_crash_path = std::path::Path::new(TEST_CRASH_PATH).join(TARGET);

        // this is default behavior
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "",
            trdelnik_client::HFUZZ_WORKSPACE_DEFAULT,
        );

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior where path is specified within env variable HFUZZ_WORKSPACE, but not within -W HFUZZ_RUN_ARGS
        let (crash_dir, ext) =
            trdelnik_client::get_crash_dir_and_ext(ROOT, TARGET, "-Q -e", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior as above
        let (crash_dir, ext) =
            trdelnik_client::get_crash_dir_and_ext(ROOT, TARGET, "-Q -e crash", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE has default value however -W is set
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q -W /home/crash -e crash",
            trdelnik_client::HFUZZ_WORKSPACE_DEFAULT,
        );

        let expected_crash_path = std::path::Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE is set and -W is also set
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crash -e crash",
            TEST_CRASH_PATH,
        );
        let expected_crash_path = std::path::Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE is set and -W is also set
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crash/foo/bar/dead/beef -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new("/home/crash/foo/bar/dead/beef");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        // HFUZZ_WORKSPACE is set and -W is also set, this time with relative path
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q -W ../crash -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        // HFUZZ_WORKSPACE is set and -W is also set, this time with relative path
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q -W ../../dead/beef/crash -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../../dead/beef/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash ../crash -e crash",
            trdelnik_client::HFUZZ_WORKSPACE_DEFAULT,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace option , which have precedence before
        // HFUZZ_WORKSPACE
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash ../bitcoin/to/the/moon -W /workspace -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../bitcoin/to/the/moon");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace HFUZZ_WORKSPACE
        let (crash_dir, ext) = trdelnik_client::get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crashes/we/like/solana -e crash",
            TEST_CRASH_PATH,
        );

        // If path is specified as absolute, the join will replace whole path.
        let expected_crash_path = std::path::Path::new("/home/crashes/we/like/solana");

        // let expected_crash_path = root.join("/home/crashes/we/like/solana");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");
    }
}
