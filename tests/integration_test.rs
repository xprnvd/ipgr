#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::{Command, Stdio};

    #[test]
    fn test_main() {
        // Prepare test data
        let input =
            "Sample text with IP addresses 192.168.0.1 and 2001:0db8:85a3:0000:0000:8a2e:0370:7334";
        let expected_output = "192.168.0.1\n2001:0db8:85a3:0000:0000:8a2e:0370:7334\n";

        // Run the program with the test input
        let mut command = Command::new(env!("CARGO_BIN_EXE_ipgr"))
            .arg("-s") // Use the silent flag to suppress unnecessary output
            .arg("-4") // Only print IPv4 addresses
            .arg("-6") // Only print IPv6 addresses
            .stdin(Stdio::piped()) // Enable writing to stdin
            .stdout(Stdio::piped()) // Enable capturing stdout
            .spawn()
            .expect("Failed to execute command");

        // Write the test input to the stdin of the program
        if let Some(stdin) = command.stdin.as_mut() {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        } else {
            panic!("Failed to open stdin");
        }

        // Wait for the program to finish and capture the output
        let output = command
            .wait_with_output()
            .expect("Failed to wait for command");

        // Convert the output bytes to a string
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Assert the output matches the expected output
        assert_eq!(output_str, expected_output);
    }
}
