use easy_color::{Hex, IntoHSL, IntoRGB, HSL, RGB};
use kdl::{KdlDocument, KdlNode, KdlValue, NodeKey};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

const NAME_ARG: usize = 0;
const VALUE_PROP: &str = "value";
const TYPE_ARG: usize = 0;
const UI_PROP: &str = "ui";
const CODE_PROP: &str = "code";

fn read_kdl_file(file_path: impl AsRef<Path>) -> KdlDocument {
    fs::read_to_string(file_path)
        .expect("Error reading the source file")
        .parse()
        .expect("Error parsing the source file")
}

fn get_swatch_child<'a>(child_name: &str, swatch: &'a KdlNode, i: usize) -> &'a KdlNode {
    swatch
        .children()
        .unwrap_or_else(|| panic!("Error finding the children of swatch #{}", i))
        .get(child_name)
        .unwrap_or_else(|| {
            panic!(
                "Error finding the child ({}) of the swatch #{}",
                child_name, i
            )
        })
}

fn get_swatch_child_value<'a>(
    swatch_child: &'a KdlNode,
    value: &NodeKey,
    i: usize,
) -> &'a KdlValue {
    swatch_child
        .get(value.clone())
        .unwrap_or_else(|| {
            panic!(
                "Error finding the value {:?} in the node ({}) of the swatch #{}",
                value,
                swatch_child.name(),
                i
            )
        })
        .value()
}

fn generate_html_table() {
    let mut kdl_file_path = std::env::args().nth(2).unwrap_or_default();
    let mut html_file_path = std::env::args().nth(3).unwrap_or_default();

    if kdl_file_path == String::default() && html_file_path == String::default() {
        kdl_file_path = String::from("data.kdl");
        html_file_path = String::from("html_result.html");

        println!("No custom file paths provided. Using default values:");
        println!("Source file: {}", kdl_file_path);
        println!("Target file: {}", html_file_path);
    } else if kdl_file_path == String::default() || html_file_path == String::default() {
        println!("Only a single file path was provided. Please make sure to provide both source and target file pahts");
        println!("or none of them if you want to use the default paths. For more info run the `help` subcommand.",
        );
    }

    let theme_name_arg: NodeKey = NAME_ARG.into();
    let theme_value_prop: NodeKey = VALUE_PROP.into();
    let theme_type_arg: NodeKey = TYPE_ARG.into();
    let theme_ui_prop: NodeKey = UI_PROP.into();
    let theme_code_prop: NodeKey = CODE_PROP.into();
    let kdl_file: KdlDocument = read_kdl_file(kdl_file_path);
    let mut html_string: String = String::new();

    let swatches = kdl_file
        .get("palette")
        .expect("Error finding the node (palette)")
        .children()
        .expect("Error finding the nodes (swatch)")
        .nodes();

    html_string.push_str(
        r#"<table>
    <tr><th>Midnight</th><th>Sunlight</th></tr>"#,
    );

    for (i, item) in swatches.iter().enumerate() {
        // Convert `kdlValue` values to `String` values to avoid displaying them with double quotes.
        // For some reason the `kdlValue` type implements the `fmt` method of the `Display` trait
        // always with double quotes around the acutal value. So we're forced to use the `Display`
        // implementation of normal `String`s to avoid those double quotes.
        let midnight = get_swatch_child("midnight", item, i);
        let midnight_name = get_swatch_child_value(midnight, &theme_name_arg, i).to_string();
        let midnight_value = get_swatch_child_value(midnight, &theme_value_prop, i);
        let sunlight = get_swatch_child("sunlight", item, i);
        let sunlight_name = get_swatch_child_value(sunlight, &theme_name_arg, i).to_string();
        let sunlight_value = get_swatch_child_value(sunlight, &theme_value_prop, i);
        let r#type = get_swatch_child("type", item, i);
        let type_value = get_swatch_child_value(r#type, &theme_type_arg, i).to_string();
        let info = get_swatch_child("info", item, i);
        let info_ui = get_swatch_child_value(info, &theme_ui_prop, i).to_string();
        let info_code = get_swatch_child_value(info, &theme_code_prop, i).to_string();

        let hex_midnight_value: Hex = match midnight_value {
            KdlValue::String(val) => val.as_str().try_into().unwrap(),
            _ => "#888888".try_into().unwrap(),
        };
        let rgb_midnight_value: RGB = hex_midnight_value.to_rgb();
        let hsl_midnight_value: HSL = hex_midnight_value.to_hsl();
        let char_hex_midnight_value: String = hex_midnight_value.to_string().replace("#", "");

        let hex_sunlight_value: Hex = match sunlight_value {
            KdlValue::String(val) => val.as_str().try_into().unwrap(),
            _ => "#888888".try_into().unwrap(),
        };
        let rgb_sunlight_value: RGB = hex_sunlight_value.to_rgb();
        let hsl_sunlight_value: HSL = hex_sunlight_value.to_hsl();
        let char_hex_sunlight_value: String = hex_sunlight_value.to_string().replace("#", "");

        let html_swatch = format!(
            r#"
    <tr><td>
        <img src="https://fakeimg.pl/12x12/{}/{}/" width="12" height="12" />
        <strong>{}</strong></br>
        <code>{}</code></br>
        <code>{}</code></br>
        <code>{}</code></br>
        <code>{}</code></br>
    </td><td>
        <img src="https://fakeimg.pl/12x12/{}/{}/" width="12" height="12" />
        <strong>{}</strong></br>
        <code>{}</code></br>
        <code>{}</code></br>
        <code>{}</code></br>
        <code>{}</code></br>
    </td></tr>
    <tr><td colspan="2">
        <ul>
            <li><strong>UI:</strong> {}</li>
            <li><strong>Code:</strong> {}</li>
        </ul>
    </td></tr>"#,
            char_hex_midnight_value,
            char_hex_midnight_value,
            midnight_name.trim_matches('"'),
            type_value.trim_matches('"'),
            hex_midnight_value,
            rgb_midnight_value,
            hsl_midnight_value,
            char_hex_sunlight_value,
            char_hex_sunlight_value,
            sunlight_name.trim_matches('"'),
            type_value.trim_matches('"'),
            hex_sunlight_value,
            rgb_sunlight_value,
            hsl_sunlight_value,
            info_ui.trim_matches('"'),
            info_code.trim_matches('"'),
        );
        html_string.push_str(&html_swatch);
    }

    html_string.push_str(
        r#"
</table>"#,
    );

    let mut buffer = File::create(html_file_path).expect("Error creating the target file");
    buffer
        .write_all(html_string.as_bytes())
        .expect("Error writing the target file");
}

fn display_help_info() {
    println!("TO DO: Display helpt info here...");
}

fn main() {
    let subcommand = std::env::args().nth(1).expect("No subcommand provided");

    match subcommand.as_str() {
        "help" => display_help_info(),
        "generate" => generate_html_table(),
        _ => (),
    }
}
