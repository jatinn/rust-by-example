pub fn editor(source: &str) -> String {
    format!("<div class=\"active-code\">
<button class=\"btn btn-primary run-code\" type=\"button\">Run</button>
<button class=\"btn btn-primary reset-code\" type=\"button\">Reset</button>
<div class=\"editor\">{}</div>
<div class=\"result\"></div>
</div>", escape(source))
}

fn escape(source: &str) -> String {
    let mut s = String::new();

    for chr in source.trim().chars() {
        match chr {
            '$' => s.push_str("&#36;"),
            '*' => s.push_str("&#42;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '\\' => s.push_str("&#92;"),
            '_' => s.push_str("&#95;"),
            '`' => s.push_str("&#96;"),
            chr => s.push(chr),
        }
    }

    s
}
