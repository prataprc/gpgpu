use colored::Colorize;

macro_rules! dbg_msg_type {
    ($this:expr) => {{
        use vulkano::instance::debug::MessageType;

        let val = MessageType {
            general: $this.general,
            validation: $this.general,
            performance: $this.general,
        };
        let mut types = vec![];
        if val.general {
            types.push("genr".to_string())
        }
        if val.validation {
            types.push("vald".to_string())
        }
        if val.performance {
            types.push("perf".to_string())
        }
        types
    }};
}

#[macro_export]
macro_rules! dbg_msg_severity {
    ($this:expr) => {{
        use vulkano::instance::debug::MessageSeverity;

        let val = MessageSeverity {
            error: $this.error,
            warning: $this.warning,
            information: $this.information,
            verbose: $this.verbose,
        };

        let mut severity = "NONE".to_string();
        if val.verbose {
            severity = "VERB".to_string();
        }
        if val.information {
            severity = "INFO".to_string();
        }
        if val.warning {
            severity = "WARN".to_string();
        }
        if val.error {
            severity = "EROR".to_string();
        }
        severity
    }};
}

// TODO: debug-callback not happening !!
pub fn debug_callback(msg: &vulkano::instance::debug::Message) {
    let layer = format!("[{}]", msg.layer_prefix.unwrap_or_else(|| "--NA--")).cyan();
    let typ = format!("<{}>", dbg_msg_type!(msg.ty).join(","));
    let severity = match dbg_msg_severity!(msg.severity).as_str() {
        "NONE" => format!("[NONE]").truecolor(100, 100, 100),
        "VERB" => format!("[VERB]").white(),
        "INFO" => format!("[INFO]").blue(),
        "WARN" => format!("[WARN]").yellow(),
        "EROR" => format!("[EROR]").red(),
        _ => unreachable!(),
    };

    println!("{} {:17} {} {}", layer, typ, severity, msg.description);
}

pub fn no_debug_callback(_msg: &vulkano::instance::debug::Message) {
    ()
}
