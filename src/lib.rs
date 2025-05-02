use std::io::Write;

use regex::Regex;

pub fn prefix(path: &str) {
    let s = std::fs::read_to_string(
        "/bwrcq/C/elamdf/intel16/q2_2025/bag3_ams_intel22ffl/A_MANUAL_DDR_PHY_TOP.def",
    )
    .unwrap();
    let mut out = std::fs::File::create("A_MANUAL_DDR_PHY_TOP.def").unwrap();
    let mut in_components = false;
    let mut in_nets = false;
    let re_pins =
        Regex::new(r"- (?<pin>[a-zA-Z0-9\[\]_]+) \+ NET (?<net>[a-zA-Z0-9\[\]_]+)").unwrap();
    let re_components = Regex::new(r"^- (?<component>[a-zA-Z0-9\[\]_]+)(?<rest>.*)").unwrap();
    let re_net_pin = Regex::new(
        r"\( (?<pin>[a-zA-Z_][a-zA-Z0-9\[\]_]+) (?<net>[a-zA-Z_][a-zA-Z0-9\[\]_]+) \)(?<rest>.*)$",
    )
    .unwrap();

    for line in s.lines() {
        if line.contains("DIEAREA") {
            continue;
        }

        if line.contains("COMPONENTS") {
            in_components = true;
        }
        if line.contains("NETS") && !line.contains("SPECIALNETS") {
            in_nets = true;
        }
        if line.contains("END") {
            in_components = false;
            in_nets = false;
        }
        if in_components {
            if let Some(caps) = re_components.captures(line.trim()) {
                let rest_fixed = &caps["rest"].replace("PLACED", "FIXED");
                if rest_fixed.contains("ESD") {
                    let physical_esd = rest_fixed.replace(";", "+ SOURCE DIST ;");
                    write!(
                        &mut out,
                        "- {}{}{}\n",
                        path, &caps["component"], physical_esd
                    )
                    .unwrap();
                } else {
                    if !(rest_fixed.contains("diffcheck")) {
                        write!(&mut out, "- {}{}{}\n", path, &caps["component"], rest_fixed)
                            .unwrap();
                    }
                }
            } else {
                write!(&mut out, "{}\n", line).unwrap();
            }
        } else if in_nets {
            if let Some(caps) = re_components.captures(line.trim()) {
                write!(
                    &mut out,
                    "- {}{}{}\n",
                    path, &caps["component"], &caps["rest"]
                )
                .unwrap();
            } else if let Some(caps) = re_net_pin.captures(line.trim()) {
                let first = if &caps["pin"] == "PIN" {
                    "PIN"
                } else {
                    &caps["pin"]
                };
                write!(
                    &mut out,
                    "( {}{} {}{} ){}\n",
                    path, first, path, &caps["net"], &caps["rest"]
                )
                .unwrap();
            } else {
                write!(&mut out, "{}\n", line).unwrap();
            }
        } else if let Some(caps) = re_pins.captures(line.trim()) {
            write!(
                &mut out,
                "- {}{} + NET {}{}\n",
                path, &caps["pin"], path, &caps["net"]
            )
            .unwrap();
        } else {
            write!(&mut out, "{}\n", line).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_def() {
        prefix("phy\\/ddr_phy_top_block\\/");
    }
}
