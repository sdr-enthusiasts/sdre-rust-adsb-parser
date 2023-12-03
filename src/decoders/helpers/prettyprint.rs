use std::fmt;

/// Pretty print a field if it is present.
/// If the field is not present, do nothing.

pub fn pretty_print_field_from_option<T: fmt::Display>(
    field_name: &str,
    field: &Option<T>,
    output: &mut String,
) {
    match field {
        Some(value) => {
            pretty_print_field(field_name, value, output);
        }
        None => (),
    }
}

/// Pretty print a field.
pub fn pretty_print_field<T: fmt::Display>(field_name: &str, field: &T, output: &mut String) {
    output.push_str(&format!(
        "{}{}{}\n",
        field_name,
        if !field_name.is_empty() { ": " } else { "" },
        field
    ));
}

/// Pretty print a label. The label will be centered in a 70 character line
/// with '=' on either side.
pub fn pretty_print_label(label: &str, output: &mut String) {
    // center the label in a 70 character line
    // 80 - 2 for the ':' - the length of the label
    let spaces: usize = (70 - label.len()) / 2;
    // see if we need to add an extra '=' to the end
    let extra = if (70 - label.len()) % 2 != 0 {
        '='
    } else {
        ' '
    };
    let mut buffer: String = String::new();
    for _i in 0..spaces {
        buffer.push('=');
    }

    output.push_str(&format!("{}{}{}{}\n", buffer, label, buffer, extra));
}
