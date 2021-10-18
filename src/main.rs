use std::process::Command;

use clap::{App, Arg};
use colorful::{Color, Colorful};

#[allow(unused_variables)]
fn main() -> Result<(), String> {
    let kernel_name = uname("-s")?;
    let nodename = uname("-n")?;
    let kernel_release = uname("-r")?;
    let kernel_version = uname("-v")?;
    let machine = uname("-m")?;
    let mut processor = uname("-p")?;
    let mut hardware_platform = uname("-i")?;
    let operating_system = uname("-o")?;

    match (processor.as_str(), hardware_platform.as_str()) {
        ("unknown", "unknown") => {
            processor = String::from("");
            hardware_platform = String::from("");
        }
        ("unknown", _) => {
            processor = String::from("");
        }
        (_, "unknown") => {
            hardware_platform = String::from("");
        }
        (_, _) => (),
    }

    let help_message = "Usage: unamec [OPTIONS]...
Print certain system information. With no OPTION, same as -sr.
    
  -a, --all                prints all information, in the following order,
                             except omit -p and -i if unknown:
  -s, --kernel-name        prints the kernel name
  -n, --nodename           prints the network node hostname
  -r, --kernel-release     prints the kernel release
  -v, --kernel-version     prints the kernel version
  -m, --machine            prints the machine hardware name
  -p, --processor          prints the processor type (non-portable)
  -i, --hardware-platform  prints the hardware platform (non-portable)
  -o, --operating-system   prints the operating system
  -h  --help               displays this help and exit
  -V  --version            outputs version information and exit
    
SNU (S0ra is Not Unix) colorful utils Github page: <https://github.com/S0raWasTaken/>
Uname documentation <https://www.gnu.org/software/coreutils/uname>
or available locally via: info '(coreutils) uname invocation'";

    let matches = App::new("unamec")
        .version(format!("v{}", env!("CARGO_PKG_VERSION")).as_str())
        .author("by S0raWastaken")
        .about("\n Uname, but colorful")
        .arg(
            Arg::with_name("all")
                .short("a")
                .help(
                    "Prints all information, in the following order,
                        except omit -p and -i if unknown:",
                )
                .multiple(true),
        )
        .arg(
            Arg::with_name("kernel-name")
                .short("s")
                .help("Prints the kernel name")
                .multiple(true),
        )
        .arg(
            Arg::with_name("nodename")
                .short("n")
                .help("Prints the network node hostname")
                .multiple(true),
        )
        .arg(
            Arg::with_name("kernel-release")
                .short("r")
                .help("Prints the kernel release")
                .multiple(true),
        )
        .arg(
            Arg::with_name("kernel-version")
                .short("v")
                .help("Prints the kernel version")
                .multiple(true),
        )
        .arg(
            Arg::with_name("machine")
                .short("m")
                .help("print the machine hardware name")
                .multiple(true),
        )
        .arg(
            Arg::with_name("processor")
                .short("p")
                .help("print the processor type (non-portable)")
                .multiple(true),
        )
        .arg(
            Arg::with_name("hardware-platform")
                .short("i")
                .help("print the hardware platform (non-portable)")
                .multiple(true),
        )
        .arg(
            Arg::with_name("operating-system")
                .short("o")
                .help("print the operating system")
                .multiple(true),
        )
        .help(help_message)
        .get_matches();

    let mut output = String::new();

    if matches.is_present("all") {
        output = format!(
            "{} {} {} {} {} {} {} {}",
            kernel_name.color(Color::Purple1b),
            nodename.color(Color::Yellow),
            kernel_release.color(Color::Red),
            kernel_version.color(Color::Blue),
            machine.color(Color::Orange1),
            processor.color(Color::Green),
            hardware_platform,
            operating_system.color(Color::Black)
        );
        println!("{}", output.replace("  ", ""));
        return Ok(());
    }

    if matches.is_present("kernel-name") {
        output = format!("{}", kernel_name.as_str().color(Color::Purple1b));
    }
    if matches.is_present("nodename") {
        output = format!("{} {}", output, nodename.color(Color::Yellow));
    }
    if matches.is_present("kernel-release") {
        output = format!("{} {}", output, kernel_release.as_str().color(Color::Red));
    }
    if matches.is_present("kernel-version") {
        output = format!("{} {}", output, kernel_version.color(Color::Blue));
    }
    if matches.is_present("machine") {
        output = format!("{} {}", output, machine.color(Color::Orange1));
    }
    if matches.is_present("processor") {
        output = format!("{} {}", output, processor.color(Color::Green));
    }
    if matches.is_present("hardware-platform") {
        output = format!("{} {}", output, hardware_platform);
    }
    if matches.is_present("operating-system") {
        output = format!("{} {}", output, operating_system.color(Color::Black));
    }

    if output.is_empty() {
        output = format!("{} {}", kernel_name.color(Color::Purple1b), kernel_release.color(Color::Red))
    }
    println!("{}", output.trim());
    Ok(())
}

fn uname(option: &str) -> Result<String, String> {
    let child = Command::new("uname")
        .arg(option)
        .output()
        .map_err(|_| String::from("Couldn't get uname info"))?;

    Ok(String::from_utf8(child.stdout)
        .map_err(|_| String::from("Couldn't convert from UTF8"))?
        .trim()
        .to_string())
}
